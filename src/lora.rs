use crate::enums::mode_type::ModeType;
use crate::status;
use crate::status::Status;
use crate::enums::program_command::ProgramCommand;
use crate::utility::configuration::Configuration;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ResponseStatus {
    code: Status
}

impl ResponseStatus {
    pub fn new(status: Status) -> ResponseStatus {
        ResponseStatus {
            code: status
        }
    }

    fn get_response_description(&self) -> String {
        status::get_response_description_by_params(self.clone().code)
    }
}

pub struct ResponseStructContainer {
    // void *data;
    rssi: u8,
    status: ResponseStatus
    // void close()
}

impl ResponseStructContainer {
    pub fn new(status: ResponseStatus) -> ResponseStructContainer {
        ResponseStructContainer {
            rssi: 0,
            status
        }
    }
}

pub struct ConfigurationResponse {
    status: Status,
    configuration: Option<Configuration>
}

impl ConfigurationResponse {
    pub fn new(status: Status, configuration: Option<Configuration>) -> ConfigurationResponse {
        ConfigurationResponse {
            status,
            configuration
        }
    }

    pub fn get_status(&self) -> Status {
        return self.status.clone()
    }

    pub fn get_configuration(&self) -> Option<Configuration> {
        return self.configuration.clone()
    }
}

pub struct ResponseContainer {
    data: String,
    rssi: u8,
    status: Status
}

pub const NOT_SET: i8 = -1;

use crate::uart::UartBpsRate;
pub struct LoRa {
    tx_e220_pin: i8,
    rx_e220_pin: i8,
    aux_pin: i8,

    m0_pin: i8,
    m1_pin: i8,

    bps_rate: UartBpsRate,
    mode: ModeType
}

impl LoRa {
    pub fn new() -> LoRa {
        LoRa {
            tx_e220_pin: NOT_SET,
            rx_e220_pin: NOT_SET,
            aux_pin: NOT_SET,

            m0_pin: NOT_SET,
            m1_pin: NOT_SET,

            bps_rate: UartBpsRate::UartBpsRate9600,
            mode: ModeType::MODE_0_NORMAL
        }
    }

    pub fn get_configuration(&self) -> ConfigurationResponse {
        let response: ConfigurationResponse;
        let status = self.check_uart_configuration(ModeType::MODE_3_PROGRAM);

        if status != Status::E220Success {
            response = ConfigurationResponse::new(status, None);
            return response;
        }

        let _prev_mode: ModeType;

        todo!();

        //return response
    }

    fn set_configuration(_configuration: Configuration, _save_type: ProgramCommand) -> ResponseStatus {   // default = WRITE_CFG_PWR_DWN_LOSE
        todo!()
    }

    fn check_uart_configuration(&self, mode: ModeType) -> Status {
        if mode == ModeType::MODE_3_PROGRAM && self.bps_rate != UartBpsRate::UartBpsRate9600 {
            return Status::ErrE220WrongUartConfig;
        }
        return Status::E220Success
    }

    fn get_mode(&self) -> ModeType {
        return self.mode.clone()
    }

    fn set_mode(_mode: ModeType) -> Status {
        todo!()
    }

    fn managed_delay(timeout: Duration) {
        let start = Instant::now();
        while start.elapsed() < timeout {}
    }

    fn pin_is_set(pin: i8) -> bool {
        if pin == NOT_SET {
            false
        } else {
            true
        }
    }
}