use crate::enums::mode_type::ModeType;
use crate::status;
use crate::status::E220Error;
use crate::enums::program_command::ProgramCommand;
use crate::enums::packet_length::PacketLength;
use crate::enums::register_address::RegisterAddress;
use crate::utility::configuration::Configuration;
use std::time::{Duration, Instant};

#[cfg(feature = "default")]
use rppal::gpio::{Gpio, InputPin, OutputPin};
#[cfg(feature = "default")]
use rppal::uart::{Uart, Parity};

#[cfg(not(feature = "default"))]
use crate::mock::gpio::{Gpio, InputPin, OutputPin};
#[cfg(not(feature = "default"))]
use crate::mock::uart::{Uart, Parity};

#[derive(Debug, Clone)]
pub struct ResponseStatus {
    code: E220Error
}

impl ResponseStatus {
    pub fn new(status: E220Error) -> ResponseStatus {
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
    status: E220Error,
    configuration: Option<Configuration>
}

pub struct ResponseContainer {
    data: String,
    rssi: u8,
    status: E220Error
}

pub const UNINITIALIZED: i8 = -1;
pub const MAX_SIZE_TX_PACKET: u8 = 200;

use crate::uart::UartBpsRate;
pub struct LoRa {
    aux_pin: Option<InputPin>,

    m0_pin: OutputPin,
    m1_pin: OutputPin,

    uart: Uart,

    bps_rate: UartBpsRate,
    mode: ModeType
}

impl LoRa {
    pub fn new(m0_pin: u8, m1_pin: u8) -> LoRa {
        let gpio = Gpio::new().expect("GPIO failed to initialize!");

        LoRa {
            aux_pin: None,

            m0_pin: gpio.get(m0_pin).expect("M0 pin failed to be fetched!").into_output(),
            m1_pin: gpio.get(m1_pin).expect("M1 pin failed to be fetched!").into_output(),

            // might need changes
            uart: Uart::new(9600, Parity::None, 8, 1).expect("UART failed to initialize!"),

            bps_rate: UartBpsRate::UartBpsRate9600,
            mode: ModeType::MODE_0_NORMAL
        }
    }

    pub fn with_aux(m0_pin: u8, m1_pin: u8, aux_pin: u8) -> LoRa {
        let gpio = Gpio::new().expect("GPIO failed to initialize!");

        LoRa {
            aux_pin: Some(gpio.get(aux_pin).expect("AUX pin failed to be fetched!").into_input()),

            m0_pin: gpio.get(m0_pin).expect("M0 pin failed to be fetched!").into_output(),
            m1_pin: gpio.get(m1_pin).expect("M1 pin failed to be fetched!").into_output(),

            // might need changes
            uart: Uart::new(9600, Parity::None, 8, 1).expect("UART failed to initialize!"),

            bps_rate: UartBpsRate::UartBpsRate9600,
            mode: ModeType::MODE_0_NORMAL
        }
    }

    pub fn get_configuration(&mut self) -> Result<Configuration, E220Error> {
        self.check_uart_configuration(ModeType::MODE_3_PROGRAM)?;

        let prev_mode = self.get_mode();
        self.set_mode(ModeType::MODE_3_PROGRAM)?;

        self.write_program_command(
            ProgramCommand::ReadConfiguration,
            RegisterAddress::Configuration,
            PacketLength::Configuration
        );

        let result = self.receive_struct(11);  // has to be verified

        self.print_parameters();

        self.set_mode(prev_mode)?;

        let data = result?;
        let configuration = Configuration::from_bytes(&data);

        if configuration.get_command() == ProgramCommand::WrongFormat as u8 {
            return Err(E220Error::WrongFormat);
        }

        // change Configuration struct to use enums instead
        if configuration.get_command() != ProgramCommand::ReturnedCommand as u8
            || configuration.get_starting_address() != RegisterAddress::Configuration as u8
            || configuration.get_length() != PacketLength::Configuration as u8
        {
            return  Err(E220Error::HeadNotRecognized);
        }

        Ok(configuration)
    }

    fn set_configuration(&mut self, mut configuration: Configuration, permanent: bool) -> Result<(), E220Error> {
        self.check_uart_configuration(ModeType::MODE_3_PROGRAM)?;

        let prev_mode = self.get_mode();

        self.set_mode(ModeType::MODE_3_PROGRAM)?;

        if permanent {
            configuration.set_command(ProgramCommand::WriteCfgPwrDwnSave);
        } else {
            configuration.set_command(ProgramCommand::WriteCfgPwrDwnLose);
        }

        configuration.set_starting_address(RegisterAddress::Configuration);
        configuration.set_length(PacketLength::Configuration);

        let data = configuration.to_bytes();
        self.send_struct(data, 11)?;    // again, verify the size

        let received_data = self.receive_struct(11)?;

        self.print_parameters();

        self.set_mode(prev_mode)?;

        let received_configuration = Configuration::from_bytes(&received_data);

        // could compare the configuration objects instead
        if received_configuration.get_command() == ProgramCommand::WrongFormat as u8 {
            return Err(E220Error::WrongFormat)
        }

        if received_configuration.get_command() != ProgramCommand::ReturnedCommand as u8
            || received_configuration.get_starting_address() != RegisterAddress::Configuration as u8
            || received_configuration.get_length() != PacketLength::Configuration as u8
        {
            return Err(E220Error::HeadNotRecognized)
        }

        Ok(())
    }

    fn check_uart_configuration(&self, mode: ModeType) -> Result<(), E220Error> {
        if mode == ModeType::MODE_3_PROGRAM && self.bps_rate != UartBpsRate::UartBpsRate9600 {
            return Err(E220Error::WrongUartConfig);
        }
        Ok(())
    }

    fn get_mode(&self) -> ModeType {
        self.mode.clone()
    }

    fn set_mode(&mut self, mode: ModeType) -> Result<(), E220Error> {
        let duration = 40;
        Self::managed_delay(Duration::from_millis(duration));

        match mode {
            ModeType::MODE_0_NORMAL => {
                self.m0_pin.set_low();
                self.m1_pin.set_low();
                println!("MODE: NORMAL")
            },
            ModeType::MODE_1_WOR_TRANSMITTER => {
                self.m0_pin.set_high();
                self.m1_pin.set_low();
                println!("MODE: WOR TRANSMITTING")
            },
            ModeType::MODE_2_WOR_RECEIVER => {
                self.m0_pin.set_low();
                self.m1_pin.set_high();
                println!("MODE: WOR RECEIVING")
            },
            ModeType::MODE_3_CONFIGURATION => {
                self.m0_pin.set_high();
                self.m1_pin.set_high();
                println!("MODE: SLEEP CONFIG")
            },
            _ => return Err(E220Error::InvalidParam)
        }

        Self::managed_delay(Duration::from_millis(duration));

        let duration = Duration::from_secs(1);
        self.wait_complete_response(duration, duration)?;

        self.mode = mode;

        Ok(())
    }

    fn print_parameters(&self) {
        todo!()
    }

    fn write_program_command(&mut self, cmd: ProgramCommand, addr: RegisterAddress, pl: PacketLength) -> bool {
        let command = vec![cmd as u8, addr as u8, pl as u8];
        let size = self.uart.write(&command).expect("Failed to write to UART");

        println!("Bytes written: {size}");

        Self::managed_delay(Duration::from_millis(50));

        size != 2
    }

    fn receive_struct(&mut self, expected_size: usize) -> Result<Vec<u8>, E220Error> {
        let mut data = Vec::new();
        let read_bytes = self.uart.read(&mut data).expect("Failed to read from UART");

        println!("Available buffer: {read_bytes}");
        println!("Expected size: {expected_size}");

        if read_bytes != expected_size {
            if read_bytes == 0 {
                return Err(E220Error::NoResponseFromDevice)
            }
            return Err(E220Error::DataSizeNotMatch)
        }

        let duration = Duration::from_secs(1);
        self.wait_complete_response(duration, duration)?;

        Ok(data)
    }

    fn send_struct(&mut self, data: Vec<u8>, size: usize) -> Result<(), E220Error> {
        if size > MAX_SIZE_TX_PACKET as usize {
            return Err(E220Error::PacketTooBig)
        }

        let written_bytes = self.uart.write(&data).expect("Failed to write to UART");

        println!("Sending data...");
        println!("Data size: {written_bytes}");
        println!("Expected size: {size}");

        if written_bytes != size {
            if written_bytes == 0 {
                return Err(E220Error::NoResponseFromDevice)
            }
            return Err(E220Error::DataSizeNotMatch)
        }

        let duration = Duration::from_secs(5);
        self.wait_complete_response(duration, duration)?;

        println!("Clearing UART buffer...");
        self.clear_uart_buffer();

        Ok(())
    }

    fn clear_uart_buffer(&mut self) {
        let mut buffer = [0u8; 256]; // clear the buffer in 256 byte chunks

        loop {
            let read_bytes = self.uart.read(&mut buffer).expect("Failed to clear UART");
            if read_bytes == 0 {
                return;
            }
        }
    }

    fn managed_delay(timeout: Duration) {
        let start = Instant::now();
        while start.elapsed() < timeout {}
    }

    fn wait_complete_response(&self, timeout: Duration, wait_no_aux: Duration) -> Result<(), E220Error> {
        let start = Instant::now();

        if let Some(aux) = &self.aux_pin {
            while aux.is_low() {
                if start.elapsed() > timeout {
                    println!("Timeout error!");
                    return Err(E220Error::Timeout)
                }
            }

            println!("AUX HIGH")
        } else {
            Self::managed_delay(wait_no_aux);
            println!("Wait no AUX pin!")
        }

        let duration = 20;

        Self::managed_delay(Duration::from_millis(duration));
        println!("Complete!");

        Ok(())
    }

    fn pin_is_set(pin: i8) -> bool {
        if pin == UNINITIALIZED {
            false
        } else {
            true
        }
    }
}