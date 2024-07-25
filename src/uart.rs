pub enum UartParity {
    Mode00_8N1 = 0b00,
    Mode01_8O1 = 0b01,
    Mode10_8E1 = 0b10,
    Mode11_8N1 = 0b11,
}

pub fn get_uart_parity_description_by_params(_uart_parity: UartParity) -> String {
    todo!()
}

pub enum UartBpsType {
    UartBps1200 = 0b000,
    UartBps2400 = 0b001,
    UartBps4800 = 0b010,
    UartBps9600 = 0b011,
    UartBps19200 = 0b100,
    UartBps38400 = 0b101,
    UartBps57600 = 0b110,
    UartBps115200 = 0b111,
}

#[derive(PartialEq)]
pub enum UartBpsRate {
    UartBpsRate1200 = 1200,
    UartBpsRate2400 = 2400,
    UartBpsRate4800 = 4800,
    UartBpsRate9600 = 9600,
    UartBpsRate19200 = 19200,
    UartBpsRate38400 = 38400,
    UartBpsRate57600 = 57600,
    UartBpsRate115200 = 115200
}

pub fn get_uart_baud_rate_description_by_params(_uart_baud_rate: UartBpsType) -> String {
    todo!()
}
