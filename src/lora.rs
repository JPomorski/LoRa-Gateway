use std::mem::size_of;
use crate::enums::mode_type::ModeType;
use crate::status;
use crate::status::Status;
use crate::enums::program_command::ProgramCommand;
use crate::enums::packet_length::PacketLength;
use crate::enums::register_address::RegisterAddress;
use crate::utility::configuration::Configuration;
use std::time::{Duration, Instant};

#[cfg(feature = "default")]
use rppal::gpio::Gpio;
#[cfg(feature = "default")]
use rppal::uart::Uart;

#[cfg(not(feature = "default"))]
use crate::mock::gpio::Gpio;
#[cfg(not(feature = "default"))]
use crate::mock::uart::Uart;

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

// impl ConfigurationResponse {
//     pub fn new(status: Status, configuration: Option<Configuration>) -> ConfigurationResponse {
//         ConfigurationResponse {
//             status,
//             configuration
//         }
//     }
//
//     pub fn get_status(&self) -> Status {
//         return self.status.clone()
//     }
//
//     pub fn get_configuration(&self) -> Option<Configuration> {
//         return self.configuration.clone()
//     }
// }

pub struct ResponseContainer {
    data: String,
    rssi: u8,
    status: Status
}

pub const UNINITIALIZED: i8 = -1;

use crate::uart::UartBpsRate;
pub struct LoRa {
    tx_e220_pin: i8,
    rx_e220_pin: i8,
    aux_pin: i8,

    m0_pin: i8,
    m1_pin: i8,

    uart: Uart,

    bps_rate: UartBpsRate,
    mode: ModeType
}

impl LoRa {
    pub fn new(uart: Uart) -> LoRa {
        LoRa {
            tx_e220_pin: UNINITIALIZED,
            rx_e220_pin: UNINITIALIZED,
            aux_pin: UNINITIALIZED,

            m0_pin: UNINITIALIZED,
            m1_pin: UNINITIALIZED,

            uart,

            bps_rate: UartBpsRate::UartBpsRate9600,
            mode: ModeType::MODE_0_NORMAL
        }
    }

    pub fn get_configuration(&mut self) -> Result<Configuration, Status> {
        let status = self.check_uart_configuration(ModeType::MODE_3_PROGRAM);

        if status != Status::E220Success {
            return Err(status)
        }

        let prev_mode = self.mode.clone();
        let status = self.set_mode(ModeType::MODE_3_PROGRAM);

        if status != Status::E220Success {
            return Err(status)
        }

        self.write_program_command(
            ProgramCommand::ReadConfiguration,
            RegisterAddress::RegAddressCfg,
            PacketLength::PlConfiguration
        );

        // change to return byte array
        let mut data = vec![0u8];
        let status = self.receive_struct(&mut data, size_of::<Configuration>());  // has to be verified

        if status != Status::E220Success {
            self.set_mode(prev_mode);

            return Err(status)
        }

        self.print_parameters();

        let status = self.set_mode(prev_mode);

        if status != Status::E220Success {
            return Err(status)
        }

        // use byte array to create a configuration
        let configuration = Configuration::from_bytes(&data);

        if configuration.get_command() == ProgramCommand::WrongFormat as u8 {
            let status = Status::ErrE220WrongFormat;
            return Err(status)
        }

        // change Configuration struct to use enums instead
        if configuration.get_command() != ProgramCommand::ReturnedCommand as u8
            || configuration.get_starting_address() != RegisterAddress::RegAddressCfg as u8
            || configuration.get_length() != PacketLength::PlConfiguration as u8
        {
            let status = Status::ErrE220HeadNotRecognized;
            return Err(status)
        }

        Ok(configuration)
    }

    fn set_configuration(_configuration: Configuration, _save_type: ProgramCommand) -> ResponseStatus {   // default = WRITE_CFG_PWR_DWN_LOSE
        todo!()
    }

    fn check_uart_configuration(&self, mode: ModeType) -> Status {
        if mode == ModeType::MODE_3_PROGRAM && self.bps_rate != UartBpsRate::UartBpsRate9600 {
            return Status::ErrE220WrongUartConfig;
        }
        Status::E220Success
    }

    fn get_mode(&self) -> ModeType {
        self.mode.clone()
    }

    fn set_mode(&mut self, mode: ModeType) -> Status {
        let duration = 40;
        Self::managed_delay(Duration::from_millis(duration));

        if self.m0_pin == UNINITIALIZED && self.m1_pin == UNINITIALIZED {
            println!("The M0 and M1 pins are not set!")
        } else {
            let gpio = Gpio::new().expect("GPIO failed to initialize!");
            let mut m0 = gpio.get(self.m0_pin as u8).expect("M0 pin failed to be fetched!").into_output();
            let mut m1 = gpio.get(self.m1_pin as u8).expect("M1 pin failed to be fetched!").into_output();

            match mode {
                ModeType::MODE_0_NORMAL => {
                    m0.set_low();
                    m1.set_low();
                    println!("MODE: NORMAL")
                },
                ModeType::MODE_1_WOR_TRANSMITTER => {
                    m0.set_high();
                    m1.set_low();
                    println!("MODE: WOR TRANSMITTING")
                },
                ModeType::MODE_2_WOR_RECEIVER => {
                    m0.set_low();
                    m1.set_high();
                    println!("MODE: WOR RECEIVING")
                },
                ModeType::MODE_3_CONFIGURATION => {
                    m0.set_high();
                    m1.set_high();
                    println!("MODE: SLEEP CONFIG")
                },
                _ => return Status::ErrE220InvalidParam
            }
        }

        Self::managed_delay(Duration::from_millis(duration));

        let duration = Duration::from_secs(1);
        let result = self.wait_complete_response(duration, duration);

        if result == Status::E220Success {
            self.mode = mode;
        }

        result
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

    fn receive_struct(&mut self, buffer: &mut [u8], expected_size: usize) -> Status {
        let read_bytes = self.uart.read(buffer).expect("Failed to read from UART");

        println!("Available buffer: {read_bytes}");
        println!("Expected size: {expected_size}");

        if read_bytes != expected_size {
            if read_bytes == 0 {
                return Status::ErrE220NoResponseFromDevice
            }
            return Status::ErrE220DataSizeNotMatch
        }

        let duration = Duration::from_secs(1);
        self.wait_complete_response(duration, duration)
    }

    fn managed_delay(timeout: Duration) {
        let start = Instant::now();
        while start.elapsed() < timeout {}
    }

    fn wait_complete_response(&self, timeout: Duration, wait_no_aux: Duration) -> Status {
        let start = Instant::now();

        if self.aux_pin != UNINITIALIZED {
            let gpio = Gpio::new().expect("GPIO failed to initialize!");
            let aux = gpio.get(self.aux_pin as u8).expect("AUX pin failed to be fetched!").into_input();

            while aux.is_low() {
                if start.elapsed() > timeout {
                    println!("Timeout error!");
                    return Status::ErrE220Timeout
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

        Status::E220Success
    }

    fn pin_is_set(pin: i8) -> bool {
        if pin == UNINITIALIZED {
            false
        } else {
            true
        }
    }
}