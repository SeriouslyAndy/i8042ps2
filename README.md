# 8042 microcontroller integration for x86
@author: <JavaKet> Mitran Andrei
## 1) Things I must follow

- Define and implement register abstractions for ports `0x60` (data) and `0x64` (status/command)
- Implement a polled input function (no interrupts) to read PS/2 scan codes
- Decode and display keyboard input characters
- Test driver functionality in QEMU
- Keyboard + Mouse driver

## 2) Documentation

Assign symbols to each reference:
- §  https://wiki.osdev.org/index.php?title=I8042_PS/2_Controller&utm_source=chatgpt.com
- $  https://www.dosdays.co.uk/media/intel/Intel%208042.pdf
- &  https://os.phil-opp.com/
- !  https://book.tockos.org/development/peripheral.html
- @  https://book.tockos.org/development/hil.html
- %  https://github.com/rust-embedded-community/pc-keyboard
- ^  https://github.com/rust-embedded-community/pc-keyboard/blob/main/src/lib.rs
- ~  https://github.com/Halicery/8042/blob/main/8042_PS2_INTERN.TEXT

Another useful link:
- https://github.com/torvalds/linux/blob/master/drivers/input/serio/libps2.c
  - We use this to research detection of every keyboard

Gathered Information:
- I/O Ports: Uses ports `0x60` (data) and `0x64` (status/command) [§]
- Controller Overview: Intel 8042 supports bi-directional keyboard (IRQ1) and mouse (IRQ12) channels on x86 [§]
- Translation Mode: Default translates scan-code set 2 → set 1; disable to receive raw sets 2/3 [§]
- USB Legacy Support: BIOS may emulate PS/2 over USB—disable emulation before PS/2 init [§]

### Status Register (0x64 read)
The Status Register flags (bit = offset):
- **Bit 0** – Output buffer status (0 = empty, 1 = full)
- **Bit 1** – Input buffer status (0 = empty, 1 = full)
- **Bit 2** – System Flag (POST passed)
- **Bit 3** – Command/data (0 = data, 1 = command)
- **Bit 4** – Unknown (chipset-specific)
- **Bit 5** – Unknown (chipset-specific)
- **Bit 6** – Time-out error (0 = no error, 1 = timeout)
- **Bit 7** – Parity error (0 = no error, 1 = parity)

Defined as:
```
Status [
    OBFull     OFFSET(0) NUMBITS(1) [],  // bit 0
    IBFull     OFFSET(1) NUMBITS(1) [],  // bit 1
    SysFlag    OFFSET(2) NUMBITS(1) [],  // bit 2
    CmdData    OFFSET(3) NUMBITS(1) [],  // bit 3
    TimeoutErr OFFSET(6) NUMBITS(1) [],  // bit 6
    ParityErr  OFFSET(7) NUMBITS(1) [],  // bit 7
]
```

## 3) Commands

Command Byte | Meaning                                                            | Response Byte
------------ | ------------------------------------------------------------------ | -------------
0x20         | Read “byte 0” from internal RAM                                    | Controller Configuration Byte
0x21–0x3F    | Read “byte N” (N = cmd & 0x1F) from internal RAM                    | Unknown
0x60         | Write next byte to “byte 0” of internal RAM (Config Byte)          | None
0x61–0x7F    | Write next byte to “byte N” of internal RAM                        | None
0xA7         | Disable second PS/2 port (if supported)                            | None
0xA8         | Enable second PS/2 port (if supported)                             | None
0xA9         | Test second PS/2 port (if supported)                               | 0x00 = pass
0xAA         | Test PS/2 Controller                                               | 0x55 = pass, 0xFC = fail
0xAB         | Test first PS/2 port                                               | 0x00 = pass
0xAC         | Diagnostic dump (read all internal RAM)                            | Unknown
0xAD         | Disable first PS/2 port                                            | None
0xAE         | Enable first PS/2 port                                             | None
0xC0         | Read controller input port                                         | Unknown
0xC1         | Copy bits 0–3 of input port → status bits 4–7                      | None
0xC2         | Copy bits 4–7 of input port → status bits 4–7                      | None
0xD0         | Read Controller Output Port                                        | Unknown
0xD1         | Write next byte to Controller Output Port                          | None
0xD2         | Write next byte to first PS/2 port output buffer (emulate input)   | None
0xD3         | Write next byte to second PS/2 port output buffer (emulate input)  | None
0xD4         | Write next byte to second PS/2 port input buffer (sends to device)| None
0xF0–0xFF    | Pulse output lines low for 6 ms (mask bits 0–3; bit 0 = reset line)| None

> Bits marked “only if dual-port supported” should be treated as undefined on single-port controllers.

## 4) Configuration Byte (0x20/0x60)
Read/Write via commands 0x20 and 0x60:

Bit | Name                    | Meaning
--- | ----------------------- | ---------------------------------------------
0   | First PS/2 IRQ Enable   | 1 = keyboard IRQ enabled; 0 = disabled
1   | Second PS/2 IRQ Enable  | 1 = mouse IRQ enabled; 0 = disabled*
2   | System Flag             | 1 = POST passed; 0 = invalid
3   | Reserved                | Must be 0
4   | First PS/2 Clock        | 1 = disabled; 0 = enabled
5   | Second PS/2 Clock       | 1 = disabled; 0 = enabled*
6   | Translation Enable      | 1 = translate SC2→SC1; 0 = raw codes
7   | Reserved                | Must be 0

```rust
Config [
    Intr1Enable   OFFSET(0) NUMBITS(1) [],
    Intr2Enable   OFFSET(1) NUMBITS(1) [],
    SysFlag       OFFSET(2) NUMBITS(1) [],
    Reserved3     OFFSET(3) NUMBITS(1) [],
    Disable1Clock OFFSET(4) NUMBITS(1) [],
    Disable2Clock OFFSET(5) NUMBITS(1) [],
    Translation   OFFSET(6) NUMBITS(1) [],
    Reserved7     OFFSET(7) NUMBITS(1) [],
]
```

*only if dual-port supported

## 5) Detecting PS/2 Devices

Full detection sequence:
1. Send Disable Scanning (0xF5) → await ACK (0xFA)
2. Send Identify (0xF2) → await ACK (0xFA)
3. Read up to 2 ID bytes (timeout if only one)
4. Send Enable Scanning (0xF4)

Partial ID responses:
- none → Ancient AT keyboard
- 0x00 → Standard PS/2 mouse
- 0x03 → Scroll-wheel mouse
- 0x04 → 5-button mouse
- 0xAB, 0x83 → MF2 keyboard
- … (see Linux `libps2.c` for full list)

## 6) Next Steps
- Driver struct and init sequence
- Implement keyboard driver
- Mouse driver
