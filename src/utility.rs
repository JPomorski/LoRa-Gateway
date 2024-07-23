// #elif defined(FREQUENCY_868)
pub const OPERATING_FREQUENCY: u16 = 850;

pub mod speed {
    pub struct Speed {
        air_data_rate: u8,
        uart_parity: u8,
        uart_baud_rate: u8
    }

    impl Speed {
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
    pub struct TransmissionMode {
        wor_period: u8,
        reserved_2: u8,
        enable_lbt: u8,
        reserved: u8,

        fixed_transmission: u8,
        enable_rssi: u8
    }

    impl TransmissionMode {
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
    pub struct Opt {
        transmission_power: u8,
        reserved: u8,   // might be obsolete
        rssi_ambient_noise: u8,
        sub_packet_setting: u8
    }

    impl Opt {
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

pub mod configuration {
    pub struct Configuration {
        command: u8,
        starting_address: u8,
        length: u8,

        add_h: u8,
        add_l: u8,

        //struct Speed;
        //struct Option;

        chan: u8    // default = 0

        //struct TransmissionMode;
        //struct Crypt;
    }

    impl Configuration {
        pub fn get_channel_description(&self) -> String {
            todo!()
        }
    }
}