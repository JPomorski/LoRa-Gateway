#[derive(Debug, Clone)]
pub enum Status {
    E220Success = 1,
    ErrE220Unknown,
    ErrE220NotSupport,
    ErrE220NotImplement,
    ErrE220NotInitial,
    ErrE220InvalidParam,
    ErrE220DataSizeNotMatch,
    ErrE220BufTooSmall,
    ErrE220Timeout,
    ErrE220Hardware,
    ErrE220HeadNotRecognized,
    ErrE220NoResponseFromDevice,
    ErrE220WrongUartConfig,
    ErrE220WrongFormat,
    ErrE220PacketTooBig
}

pub fn get_response_description_by_params(status: Status) -> String {
    match status {
        Status::E220Success => "Success".to_string(),
        Status::ErrE220Unknown => "Unknown".to_string(),
        Status::ErrE220NotSupport => "Not supported".to_string(),
        Status::ErrE220NotImplement => "Not implemented".to_string(),
        Status::ErrE220NotInitial => "Not initial".to_string(),
        Status::ErrE220InvalidParam => "Invalid param".to_string(),
        Status::ErrE220DataSizeNotMatch => "Data size does not match".to_string(),
        Status::ErrE220BufTooSmall => "Buffer too small".to_string(),
        Status::ErrE220Timeout => "Timeout".to_string(),
        Status::ErrE220Hardware => "Hardware error".to_string(),
        Status::ErrE220HeadNotRecognized => "Save mode returned not recognized".to_string(),
        Status::ErrE220NoResponseFromDevice => "No response from device (check wiring)".to_string(),
        Status::ErrE220WrongUartConfig => "Wrong UART configuration (BPS must be 9600 for configuration)".to_string(),
        Status::ErrE220WrongFormat => "Wrong format".to_string(),
        Status::ErrE220PacketTooBig => "Packet too big (max 200 bytes of data transmission)".to_string()
    }
}