// This is a temporary module containing a mock implementation of the rppal library.
// Its only purpose is to pass compilation checks on a Windows system,
// and it is not intended to be a part of the final product.

pub mod gpio {
    pub struct Gpio {

    }

    impl Gpio {
        pub fn new() -> Result<Gpio, ()> {
            Ok(Gpio {})
        }

        pub fn get(&self, _pin: u8) -> Result<Pin, ()> {
            Ok(Pin {})
        }
    }

    pub struct Pin {

    }

    impl Pin {
        pub fn into_input(self) -> InputPin {
            InputPin {}
        }

        pub fn into_output(self) -> OutputPin {
            OutputPin {}
        }
    }

    pub struct InputPin {

    }

    impl InputPin {
        pub fn is_low(&self) -> bool {
            true
        }
    }

    pub struct OutputPin {

    }

    impl OutputPin {
        pub fn set_low(&mut self) {}
        pub fn set_high(&mut self) {}
    }
}

pub mod uart {
    pub struct Uart {

    }

    impl Uart {
        pub fn new(
            _baud_rate: u32,
            _parity: Parity,
            _data_bits: u8,
            _stop_bits: u8,
        ) -> Result<Uart, ()> {
            Ok(Uart {})
        }

        pub fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, ()> {
            Ok(11)
        }

        pub fn write(&mut self, _buffer: &[u8]) -> Result<usize, ()> {
            Ok(11)
        }
    }

    pub enum Parity {
        None,
        Even,
        Odd,
        Mark,
        Space,
    }
}