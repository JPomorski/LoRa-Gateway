use crate::status;
use crate::status::Status;
use crate::enums::program_command::ProgramCommand;

#[derive(Debug, Clone)]
pub struct ResponseStatus {
    response_status: Status
}

impl ResponseStatus {
    fn get_response_description(&self) -> String {
        status::get_response_description_by_params(self.clone().response_status)
    }
}

pub struct ResponseStructContainer {

}

pub struct ResponseContainer {
    data: String,
    rssi: u8,
    status: Status
}

pub struct Configuration {
    
}

pub struct LoRa {

}

impl LoRa {
    fn get_configuration() -> ResponseStructContainer {
        todo!()
    }

    fn set_configuration(_configuration: Configuration, _save_type: ProgramCommand) {   // default = WRITE_CFG_PWR_DWN_LOSE
        todo!()
    }
}