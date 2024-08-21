#[derive(Debug, Clone, PartialEq)]
pub enum E220Error {
    Unknown = 2,
    NotSupport,
    NotImplement,
    NotInitial,
    InvalidParam,
    DataSizeNotMatch,
    BufTooSmall,
    Timeout,
    Hardware,
    HeadNotRecognized,
    NoResponseFromDevice,
    WrongUartConfig,
    WrongFormat,
    PacketTooBig
}

pub fn get_response_description_by_params(status: E220Error) -> String {
    match status {
        E220Error::Unknown => "Unknown".to_string(),
        E220Error::NotSupport => "Not supported".to_string(),
        E220Error::NotImplement => "Not implemented".to_string(),
        E220Error::NotInitial => "Not initial".to_string(),
        E220Error::InvalidParam => "Invalid param".to_string(),
        E220Error::DataSizeNotMatch => "Data size does not match".to_string(),
        E220Error::BufTooSmall => "Buffer too small".to_string(),
        E220Error::Timeout => "Timeout".to_string(),
        E220Error::Hardware => "Hardware error".to_string(),
        E220Error::HeadNotRecognized => "Save mode returned not recognized".to_string(),
        E220Error::NoResponseFromDevice => "No response from device (check wiring)".to_string(),
        E220Error::WrongUartConfig => "Wrong UART configuration (BPS must be 9600 for configuration)".to_string(),
        E220Error::WrongFormat => "Wrong format".to_string(),
        E220Error::PacketTooBig => "Packet too big (max 200 bytes of data transmission)".to_string()
    }
}