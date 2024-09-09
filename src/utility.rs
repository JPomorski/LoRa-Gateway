// #elif defined(FREQUENCY_868)
pub const OPERATING_FREQUENCY: u16 = 850;

pub mod speed {
    #[derive(Clone)]
    pub struct Speed {
        air_data_rate: u8,
        uart_parity: u8,
        uart_baud_rate: u8
    }

    impl Speed {
        pub fn new() -> Speed {
            Speed {
                air_data_rate: 0,
                uart_parity: 0,
                uart_baud_rate: 0,
            }
        }

        pub fn from_u8(air_data_rate: u8, uart_parity: u8, uart_baud_rate: u8) -> Speed {
            Speed {
                air_data_rate,
                uart_parity,
                uart_baud_rate,
            }
        }

        pub fn air_data_rate(&self) -> u8 {
            self.air_data_rate
        }

        pub fn uart_parity(&self) -> u8 {
            self.uart_parity
        }

        pub fn uart_baud_rate(&self) -> u8 {
            self.uart_baud_rate
        }

        pub fn get_air_data_rate_description(&self) -> String {
            todo!()
        }

        pub fn get_uart_parity_description(&self) -> String {
            todo!()
        }

        pub fn get_uart_baud_rate_description(&self) -> String {
            todo!()
        }
    }
}

pub mod transmission_mode {
    #[derive(Clone)]
    pub struct TransmissionMode {
        wor_period: u8,
        reserved_2: u8,
        lbt_enable: u8,
        reserved: u8,

        fixed_transmission: u8,
        rssi_enable: u8
    }

    impl TransmissionMode {
        pub fn new() -> TransmissionMode {
            TransmissionMode {
                wor_period: 0,
                reserved_2: 0,
                lbt_enable: 0,
                reserved: 0,
                fixed_transmission: 0,
                rssi_enable: 0,
            }
        }

        pub fn from_u8(wor_period: u8, lbt_enable: u8, fixed_transmission: u8, rssi_enable: u8) -> TransmissionMode {
            TransmissionMode {
                wor_period,
                reserved_2: 0,
                lbt_enable,
                reserved: 0,
                fixed_transmission,
                rssi_enable,
            }
        }

        pub fn wor_period(&self) -> u8 {
            self.wor_period
        }

        pub fn lbt_enable(&self) -> u8 {
            self.lbt_enable
        }

        pub fn fixed_transmission(&self) -> u8 {
            self.fixed_transmission
        }

        pub fn rssi_enable(&self) -> u8 {
            self.rssi_enable
        }

        pub fn get_wor_period_by_params_description(&self) -> String {
            todo!()
        }

        pub fn get_lbt_enable_byte_description(&self) -> String {
            todo!()
        }

        pub fn get_fixed_transmission_description(&self) -> String {
            todo!()
        }

        pub fn get_rssi_enable_byte_description(&self) -> String {
            todo!()
        }
    }
}

pub mod opt {
    #[derive(Clone)]
    pub struct Opt {
        transmission_power: u8,
        reserved: u8,   // might be obsolete
        rssi_ambient_noise: u8,
        sub_packet_setting: u8
    }

    impl Opt {
        pub fn new() -> Opt {
            Opt {
                transmission_power: 0,
                reserved: 0,
                rssi_ambient_noise: 0,
                sub_packet_setting: 0,
            }
        }

        pub fn from_u8(transmission_power: u8, rssi_ambient_noise: u8, sub_packet_setting: u8) -> Opt {
            Opt {
                transmission_power,
                reserved: 0,
                rssi_ambient_noise,
                sub_packet_setting,
            }
        }

        pub fn transmission_power(&self) -> u8 {
            self.transmission_power
        }

        pub fn rssi_ambient_noise(&self) -> u8 {
            self.rssi_ambient_noise
        }

        pub fn sub_packet_setting(&self) -> u8 {
            self.sub_packet_setting
        }

        pub fn get_transmission_power_description(&self) -> String {
            todo!()
        }

        pub fn get_rssi_ambient_noise_enable(&self) -> String {
            todo!()
        }

        pub fn get_sub_packet_setting_description(&self) -> String {
            todo!()
        }
    }
}

pub mod crypt {
    #[derive(Clone)]
    pub struct Crypt {
        crypt_h: u8,
        crypt_l: u8
    }

    impl Crypt {
        pub fn new() -> Crypt {
            Crypt {
                crypt_h: 0,
                crypt_l: 0
            }
        }

        pub fn from_u8(crypt_h: u8, crypt_l: u8) -> Crypt {
            Crypt {
                crypt_h,
                crypt_l,
            }
        }

        pub fn crypt_h(&self) -> u8 {
            self.crypt_h
        }

        pub fn crypt_l(&self) -> u8 {
            self.crypt_l
        }
    }
}

pub mod configuration {
    use crate::enums::PacketLength;
    use crate::enums::ProgramCommand;
    use crate::enums::RegisterAddress;
    use crate::utility::crypt::Crypt;
    use crate::utility::opt::Opt;
    use crate::utility::speed::Speed;
    use crate::utility::transmission_mode::TransmissionMode;

    #[derive(Clone)]
    pub struct Configuration {
        command: u8,
        starting_address: u8,
        length: u8,

        add_h: u8,
        add_l: u8,

        sped: Speed,
        opt: Opt,

        chan: u8,    // default = 0

        transmission_mode: TransmissionMode,
        crypt: Crypt
    }

    impl Configuration {
        pub fn new() -> Configuration {
            Configuration {
                command: 0,
                starting_address: 0,
                length: 0,

                add_h: 0,
                add_l: 0,

                sped: Speed::new(),
                opt: Opt::new(),

                chan: 0,

                transmission_mode: TransmissionMode::new(),
                crypt: Crypt::new(),
            }
        }

        pub fn from_u8(
            command: u8,
            starting_address: u8,
            length: u8,

            add_h: u8,
            add_l: u8,

            sped: Speed,
            opt: Opt,

            chan: u8,

            transmission_mode: TransmissionMode,
            crypt: Crypt
        ) -> Configuration {
            Configuration {
                command,
                starting_address,
                length,
                add_h,
                add_l,
                sped,
                opt,
                chan,
                transmission_mode,
                crypt,
            }
        }

        pub fn from_bytes(bytes: &[u8]) -> Configuration {
            Configuration {
                command: bytes[0],
                starting_address: bytes[1],
                length: bytes[2],
                add_h: bytes[3],
                add_l: bytes[4],

                sped: Speed::from_u8(
                    bytes[5] & 0b00000111,
                    (bytes[5] & 0b00011000) >> 3,
                    (bytes[5] & 0b11100000) >> 5
                ),

                opt: Opt::from_u8(
                    bytes[6] & 0b00000011,
                    (bytes[6] & 0b00100000) >> 5,
                    (bytes[6] & 0b11000000) >> 6
                ),

                chan: bytes[7],

                transmission_mode: TransmissionMode::from_u8(
                    bytes[8] & 0b00000111,
                    (bytes[8] & 0b00010000) >> 4,
                    (bytes[8] & 0b01000000) >> 6,
                    (bytes[8] & 0b10000000) >> 7
                ),
                
                crypt: Crypt::from_u8(
                    bytes[9],
                    bytes[10]
                ),
            }
        }

        pub fn to_bytes(&self) -> Vec<u8> {
            vec![
                self.command,
                self.starting_address,
                self.length,
                self.add_h,
                self.add_l,

                self.sped.air_data_rate()
                    | (self.sped.uart_parity() << 3)
                    | (self.sped.uart_baud_rate() << 5),

                self.opt.transmission_power()
                    | (self.opt.rssi_ambient_noise() << 5)
                    | (self.opt.sub_packet_setting() << 6),

                self.chan,

                self.transmission_mode.wor_period()
                    | (self.transmission_mode.lbt_enable() << 4)
                    | (self.transmission_mode.fixed_transmission() << 6)
                    | (self.transmission_mode.rssi_enable() << 7),

                self.crypt.crypt_h(),
                self.crypt.crypt_l()
            ]
        }

        pub fn command(&self) -> u8 {
            self.command
        }

        pub fn starting_address(&self) -> u8 {
            self.starting_address
        }

        pub fn length(&self) -> u8 {
            self.length
        }

        pub fn set_command(&mut self, command: ProgramCommand) {
            self.command = command.code();
        }

        pub fn set_starting_address(&mut self, starting_address: RegisterAddress) {
            self.starting_address = starting_address as u8;
        }

        pub fn set_length(&mut self, length: PacketLength) {
            self.length = length.value();
        }

        pub fn get_channel_description(&self) -> String {
            todo!()
        }

        pub fn print_parameters(&self) {
            println!("--------------------------------");

            println!("Head: 0x{:02X}, 0x{:02X}, 0x{:02X}", self.command, self.starting_address, self.length);

            println!(" ");

            println!("Add H: 0x{:02X}", self.add_h);
            println!("Add L: 0x{:02X}", self.add_l);

            println!(" ");

            println!("Channel: {}", self.chan);
            // println!("Channel: {} -> {}", self.chan, self.get_channel_description());

            println!(" ");

            println!("Speed:");
            println!("ParityBit: {:08b}", self.sped.uart_parity());
            println!("BaudRate: {:08b}", self.sped.uart_baud_rate());
            println!("AirDataRate: {:08b}", self.sped.air_data_rate());
            // println!("ParityBit: {:08b} -> {}", self.sped.uart_parity(), self.sped.get_uart_parity_description());
            // println!("BaudRate: {:08b} -> {}", self.sped.uart_baud_rate(), self.sped.get_uart_baud_rate_description());
            // println!("AirDataRate: {:08b} -> {}", self.sped.air_data_rate(), self.sped.get_air_data_rate_description());

            println!(" ");

            println!("Option:");
            println!("SubPacketSetting: {:08b}", self.opt.sub_packet_setting());
            println!("TransmissionPower: {:08b}", self.opt.transmission_power());
            println!("RSSIAmbientNoise: {:08b}", self.opt.rssi_ambient_noise());
            // println!("SubPacketSetting: {:08b} -> {}", self.opt.sub_packet_setting(), self.opt.get_sub_packet_setting_description());
            // println!("TransmissionPower: {:08b} -> {}", self.opt.transmission_power(), self.opt.get_transmission_power_description());
            // println!("RSSIAmbientNoise: {:08b} -> {}", self.opt.rssi_ambient_noise(), self.opt.get_rssi_ambient_noise_enable());

            println!(" ");

            println!("Transmission Mode:");
            println!("WORPeriod: {:08b}", self.transmission_mode.wor_period());
            println!("LBTEnable: {:08b}", self.transmission_mode.lbt_enable());
            println!("RSSIEnable: {:08b}", self.transmission_mode.rssi_enable());
            println!("FixedTransmission: {:08b}", self.transmission_mode.fixed_transmission());
            // println!("WORPeriod: {:08b} -> {}", self.transmission_mode.wor_period(), self.transmission_mode.get_wor_period_by_params_description());
            // println!("LBTEnable: {:08b} -> {}", self.transmission_mode.lbt_enable(), self.transmission_mode.get_lbt_enable_byte_description());
            // println!("RSSIEnable: {:08b} -> {}", self.transmission_mode.rssi_enable(), self.transmission_mode.get_rssi_enable_byte_description());
            // println!("FixedTransmission: {:08b} -> {}", self.transmission_mode.fixed_transmission(), self.transmission_mode.get_fixed_transmission_description());

            println!("--------------------------------");
        }
    }
}