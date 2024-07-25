use crate::enums::mode_type::ModeType;
use crate::status;
use crate::status::Status;
use crate::enums::program_command::ProgramCommand;
use crate::utility::configuration::Configuration;

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

pub struct ResponseContainer {
    data: String,
    rssi: u8,
    status: Status
}

use crate::uart::UartBpsRate;
pub struct LoRa {
    tx_e220_pin: i8,
    rx_e220_pin: i8,
    aux_pin: i8,

    m0_pin: i8,
    m1_pin: i8,

    bps_rate: UartBpsRate
}

impl LoRa {
    pub fn new() -> LoRa {
        LoRa {
            tx_e220_pin: -1,
            rx_e220_pin: -1,
            aux_pin: -1,

            m0_pin: -1,
            m1_pin: -1,

            bps_rate: UartBpsRate::UartBpsRate9600
        }
    }

    pub fn get_configuration(&self) -> ResponseStructContainer {
        let status = self.check_uart_configuration(ModeType::MODE_3_PROGRAM);
        let rc: ResponseStructContainer = ResponseStructContainer::new(ResponseStatus::new(status));

        return rc
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
}