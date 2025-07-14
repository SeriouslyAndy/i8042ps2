use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};

const DATA_PORT: u16 = 0x60; //Data port for reading/writing data
const STATUS_CMD_PORT: u16 = 0x64; //Status/Command port for reading status and sending commands

/*

I8042 PS/2 Controller
This controller is used for keyboard and mouse input!!!
author: Mitran Andrei <JavaKet>
please make sure to read the datasheet I made for more information :3

*/

///Please make sure to check https://wiki.osdev.org/index.php?title=I8042_PS/2_Controller

/// in order to understand how this works
/// The PS/2 controller is used to control the PS/2 keyboard and mouse.
/// It is a legacy component in modern systems, but it is still used for compatibility with older
/// hardware and software.
/// 
register_bitfields![u8,
    /// Status register 
    Status [
        OBFull OFFSET(0) NUMBITS(1) [],
        IBFull OFFSET(1) NUMBITS(1) [],
        SysFlag OFFSET(2) NUMBITS(1) [],
        CmdData OFFSET(3) NUMBITS(1) [],
        TimeoutErr OFFSET(6) NUMBITS(1) [],
        ParityErr OFFSET(7) NUMBITS(1) []
    ],
    /// Configuration byte 
    Config [
        Intr1Enable OFFSET(0) NUMBITS(1) [],
        Intr2Enable OFFSET(1) NUMBITS(1) [],
        SysFlag OFFSET(2) NUMBITS(1) [],
        Disable1Clock OFFSET(4) NUMBITS(1) [],
        Disable2Clock OFFSET(5) NUMBITS(1) [],
        Translation OFFSET(6) NUMBITS(1) [],
        Reserved OFFSET(7) NUMBITS(1) []
    ],
    /// Output port
      OutputPort [
        SysReset       OFFSET(0) NUMBITS(1) [],
        A20Gate        OFFSET(1) NUMBITS(1) [],
        Port2Clock     OFFSET(2) NUMBITS(1) [],
        Port2Data      OFFSET(3) NUMBITS(1) [],  
        OBF1           OFFSET(4) NUMBITS(1) [], 
        OBF2           OFFSET(5) NUMBITS(1) [],  
        Port1Clock     OFFSET(6) NUMBITS(1) [],  
        Port1Data      OFFSET(7) NUMBITS(1) []  
    ]
];

/*
| Command Byte  | Meaning                                                                 | Response Byte                                             |
| ------------- | ----------------------------------------------------------------------- | --------------------------------------------------------- |
| 0x20          | Read “byte 0” from internal RAM                                         | Configuration Byte                                        |
| 0x21–0x3F     | Read “byte N” (N = cmd & 0x1F) from internal RAM                         | Unknown                                                   |
| 0x60          | Write next byte to “byte 0” (Configuration Byte)                        | —                                                         |
| 0x61–0x7F     | Write next byte to “byte N” (N = cmd & 0x1F)                            | —                                                         |
| 0xA7          | Disable second PS/2 port                                                | —                                                         |
| 0xA8          | Enable second PS/2 port                                                 | —                                                         |
| 0xA9          | Test second PS/2 port                                                   | 0x00 = pass                                               |
| 0xAA          | Test PS/2 Controller                                                    | 0x55 = pass, 0xFC = fail                                  |
| 0xAB          | Test first PS/2 port                                                    | 0x00 = pass                                               |
| 0xAC          | Diagnostic dump (read all internal RAM)                                 | Unknown                                                   |
| 0xAD          | Disable first PS/2 port                                                 | —                                                         |
| 0xAE          | Enable first PS/2 port                                                  | —                                                         |
| 0xC0          | Read controller input port                                              | Unknown                                                   |
| 0xC1          | Copy bits 0–3 of input port → status bits 4–7                            | —                                                         |
| 0xC2          | Copy bits 4–7 of input port → status bits 4–7                            | —                                                         |
| 0xD0          | Read Controller Output Port                                              | Output Port Value                                         |
| 0xD1          | Write next byte to Controller Output Port                               | —                                                         |
| 0xD2          | Write next byte to first port output buffer (emulate input)             | —                                                         |
| 0xD3          | Write next byte to second port output buffer (emulate input)            | —                                                         |
| 0xD4          | Write next byte to second port input buffer (sends to device)           | —                                                         |
| 0xF0–0xFF     | Pulse output lines low for 6 ms (mask bits 0–3; bit 0 = reset line)      | —                                                         |
*/
register_structs! {
    I8042Ps2Registers {
        /// Control register.
        /// The 'Control' parameter constrains this register to only use
        /// fields from a certain group (defined below in the bitfields
        /// section).
        // Read internal RAM: 0x20–0x3F
        (0x20 => read_config:       ReadOnly<u8>),
        (0x21 => read_ram_1:        ReadOnly<u8>),
        (0x22 => read_ram_2:        ReadOnly<u8>),
        (0x23 => read_ram_3:        ReadOnly<u8>),
        (0x24 => read_ram_4:        ReadOnly<u8>),
        (0x25 => read_ram_5:        ReadOnly<u8>),
        (0x26 => read_ram_6:        ReadOnly<u8>),
        (0x27 => read_ram_7:        ReadOnly<u8>),
        (0x28 => read_ram_8:        ReadOnly<u8>),
        (0x29 => read_ram_9:        ReadOnly<u8>),
        (0x2A => read_ram_10:       ReadOnly<u8>),
        (0x2B => read_ram_11:       ReadOnly<u8>),
        (0x2C => read_ram_12:       ReadOnly<u8>),
        (0x2D => read_ram_13:       ReadOnly<u8>),
        (0x2E => read_ram_14:       ReadOnly<u8>),
        (0x2F => read_ram_15:       ReadOnly<u8>),
        (0x30 => read_ram_16:       ReadOnly<u8>),
        (0x31 => read_ram_17:       ReadOnly<u8>),
        (0x32 => read_ram_18:       ReadOnly<u8>),
        (0x33 => read_ram_19:       ReadOnly<u8>),
        (0x34 => read_ram_20:       ReadOnly<u8>),
        (0x35 => read_ram_21:       ReadOnly<u8>),
        (0x36 => read_ram_22:       ReadOnly<u8>),
        (0x37 => read_ram_23:       ReadOnly<u8>),
        (0x38 => read_ram_24:       ReadOnly<u8>),
        (0x39 => read_ram_25:       ReadOnly<u8>),
        (0x3A => read_ram_26:       ReadOnly<u8>),
        (0x3B => read_ram_27:       ReadOnly<u8>),
        (0x3C => read_ram_28:       ReadOnly<u8>),
        (0x3D => read_ram_29:       ReadOnly<u8>),
        (0x3E => read_ram_30:       ReadOnly<u8>),
        /// <<<<<<<<<<<<<<<<<<<<<<<<-- from 1 to 31
        /// The last RAM byte is 0x3F
        (0x3F => read_ram_31:       ReadOnly<u8>),

        // Write internal RAM: 0x60–0x7F
        (0x60 => write_config:      WriteOnly<u8>),
        /// <<<<<<<<<<<<<<<<<<<<<<<<-- from 1 to 31
        /// The last RAM byte is 0x7F, so we can use it to write the last byte
        (0x61 => write_ram_1:       WriteOnly<u8>),
        (0x62 => write_ram_2:       WriteOnly<u8>),
        (0x63 => write_ram_3:       WriteOnly<u8>),
        (0x64 => write_ram_4:       WriteOnly<u8>),
        (0x65 => write_ram_5:       WriteOnly<u8>),
        (0x66 => write_ram_6:       WriteOnly<u8>),
        (0x67 => write_ram_7:       WriteOnly<u8>),
        (0x68 => write_ram_8:       WriteOnly<u8>),
        (0x69 => write_ram_9:       WriteOnly<u8>),
        (0x6A => write_ram_10:      WriteOnly<u8>),
        (0x6B => write_ram_11:      WriteOnly<u8>),
        (0x6C => write_ram_12:      WriteOnly<u8>),
        (0x6D => write_ram_13:      WriteOnly<u8>),
        (0x6E => write_ram_14:      WriteOnly<u8>),
        (0x6F => write_ram_15:      WriteOnly<u8>),
        (0x70 => write_ram_16:      WriteOnly<u8>),
        (0x71 => write_ram_17:      WriteOnly<u8>),
        (0x72 => write_ram_18:      WriteOnly<u8>),
        (0x73 => write_ram_19:      WriteOnly<u8>),
        (0x74 => write_ram_20:      WriteOnly<u8>),
        (0x75 => write_ram_21:      WriteOnly<u8>),
        (0x76 => write_ram_22:      WriteOnly<u8>),
        (0x77 => write_ram_23:      WriteOnly<u8>),
        (0x78 => write_ram_24:      WriteOnly<u8>),
        (0x79 => write_ram_25:      WriteOnly<u8>),
        (0x7A => write_ram_26:      WriteOnly<u8>),
        (0x7B => write_ram_27:      WriteOnly<u8>),
        (0x7C => write_ram_28:      WriteOnly<u8>),
        (0x7D => write_ram_29:      WriteOnly<u8>),
        (0x7E => write_ram_30:      WriteOnly<u8>),
        (0x7F => write_ram_31:      WriteOnly<u8>),

        // Port enable/disable and tests
        (0xA7 => disable_port2:     WriteOnly<u8>),
        (0xA8 => enable_port2:      WriteOnly<u8>),
        (0xA9 => test_port2:        ReadOnly<u8, Status::Register>), // returns 0x00 if OK

        (0xAA => test_controller:   ReadOnly<u8>),  // 0x55=pass,0xFC=fail
        (0xAB => test_port1:        ReadOnly<u8>),  // 0x00=pass
        (0xAC => diagnostic_dump:   ReadOnly<u8>),  // sequential dumps on repeated reads

        (0xAD => disable_port1:     WriteOnly<u8>),
        (0xAE => enable_port1:      WriteOnly<u8>),

        // I/O port manipulation
        (0xC0 => read_input_port:   ReadOnly<u8>),
        (0xC1 => copy_in_0to3:      WriteOnly<u8>),
        (0xC2 => copy_in_4to7:      WriteOnly<u8>),

        (0xD0 => read_output_port:  ReadOnly<u8>),
        (0xD1 => write_output_port: WriteOnly<u8>),

        // Emulated device in-s
        (0xD2 => emit_port1_in:     WriteOnly<u8>),
        (0xD3 => emit_port2_in:     WriteOnly<u8>),

        // Send to 2nd Ps/2 port
        (0xD4 => write_port2_in:    WriteOnly<u8>),

        /// PS/2 Controller “Pulse Lines” Commands (0xF0–0xFF)
        ///
        /// Writing any byte in 0xF0..=0xFF to the command port (0x64) will
        /// pulse certain output lines low for 6 ms. Bits 0–3 of the byte act
        /// as a mask (0 = pulse that line, 1 = leave it high):
        /// - Bit 0 → Reset line
        /// - Bit 1 → Output line 1 (chipset-specific)
        /// - Bit 2 → Output line 2 (chipset-specific)
        /// - Bit 3 → Output line 3 (chipset-specific)
        ///
        /// Example:
        /// //-> must ignore
        /// // Pulse the reset line only:
        /// write_command_port(0xF0); // 0b1111_0000 → mask bits 4–7 unused
        ///
        // Pulse output lines (0xF0–0xFF)
        (0xF0 => pulse_line_0:  WriteOnly<u8>),
        (0xF1 => pulse_line_1:  WriteOnly<u8>),
        (0xF2 => pulse_line_2:  WriteOnly<u8>),
        (0xF3 => pulse_line_3:  WriteOnly<u8>),
        (0xF4 => pulse_line_4:  WriteOnly<u8>),
        (0xF5 => pulse_line_5:  WriteOnly<u8>),
        (0xF6 => pulse_line_6:  WriteOnly<u8>),
        (0xF7 => pulse_line_7:  WriteOnly<u8>),
        (0xF8 => pulse_line_8:  WriteOnly<u8>),
        (0xF9 => pulse_line_9:  WriteOnly<u8>),
        (0xFA => pulse_line_10: WriteOnly<u8>),
        (0xFB => pulse_line_11: WriteOnly<u8>),
        (0xFC => pulse_line_12: WriteOnly<u8>),
        (0xFD => pulse_line_13: WriteOnly<u8>),
        (0xFE => pulse_line_14: WriteOnly<u8>),
        (0xFF => pulse_line_15: WriteOnly<u8>),

        ///Detection of device presence
        /// The PS/2 controller can detect the presence of a device on the port.
        /// Time to detect some devices >:3

        (0x00 => nothing: ), // Detect device presence



        // End of block (no registers beyond 0xFF)
        (0x100 => @END),


        
    }
}
