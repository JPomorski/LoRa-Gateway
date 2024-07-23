use crate::status;
use crate::status::Status;
use crate::enums::program_command::ProgramCommand;
use crate::utility::configuration::Configuration;

#[derive(Debug, Clone)]
pub struct ResponseStatus {
    code: Status
}

impl ResponseStatus {
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

pub struct ResponseContainer {
    data: String,
    rssi: u8,
    status: Status
}

pub struct LoRa {

}

impl LoRa {
    fn get_configuration() -> ResponseStructContainer {
        todo!()
    }

    fn set_configuration(_configuration: Configuration, _save_type: ProgramCommand) -> ResponseStatus {   // default = WRITE_CFG_PWR_DWN_LOSE
        todo!()
    }
}