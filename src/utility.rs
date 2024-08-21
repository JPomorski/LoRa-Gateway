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
        enable_lbt: u8,
        reserved: u8,

        fixed_transmission: u8,
        enable_rssi: u8
    }

    impl TransmissionMode {
        pub fn new() -> TransmissionMode {
            TransmissionMode {
                wor_period: 0,
                reserved_2: 0,
                enable_lbt: 0,
                reserved: 0,
                fixed_transmission: 0,
                enable_rssi: 0,
            }
        }

        pub fn from_u8(wor_period: u8, enable_lbt: u8, fixed_transmission: u8, enable_rssi: u8) -> TransmissionMode {
            TransmissionMode {
                wor_period,
                reserved_2: 0,
                enable_lbt,
                reserved: 0,
                fixed_transmission,
                enable_rssi,
            }
        }

        pub fn get_wor_period_by_params_description() -> String {
            todo!()
        }

        pub fn get_lbt_enable_byte_description() -> String {
            todo!()
        }

        pub fn get_fixed_transmission_description() -> String {
            todo!()
        }

        pub fn get_rssi_enable_byte_description() -> String {
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

        pub fn get_transmission_power_description() -> String {
            todo!()
        }

        pub fn get_rssi_ambient_noise_enable() -> String {
            todo!()
        }

        pub fn get_sub_packet_setting() -> String {
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
    }
}

pub mod configuration {
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

        pub fn get_command(&self) -> u8 {
            self.command
        }

        pub fn get_starting_address(&self) -> u8 {
            self.starting_address
        }

        pub fn get_length(&self) -> u8 {
            self.length
        }

        pub fn get_channel_description(&self) -> String {
            todo!()
        }
    }
}