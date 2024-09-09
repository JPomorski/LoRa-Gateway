use lora_lib::enums::AirDataRate;
use lora_lib::enums::FixedTransmission;
use lora_lib::enums::LBTEnable;
use lora_lib::enums::RSSIAmbientNoiseEnable;
use lora_lib::enums::RSSIEnable;
use lora_lib::enums::SubPacketSetting;
use lora_lib::enums::TransmissionPower;
use lora_lib::enums::WORPeriod;
use lora_lib::lora::LoRa;
use lora_lib::uart::UartBpsType;
use lora_lib::uart::UartParity;
use lora_lib::utility::configuration::Configuration;
use lora_lib::utility::crypt::Crypt;
use lora_lib::utility::opt::Opt;
use lora_lib::utility::speed::Speed;
use lora_lib::utility::transmission_mode::TransmissionMode;

fn main() {
    let m0_pin = 0;
    let m1_pin = 0;

    let mut lora = LoRa::new(m0_pin, m1_pin).expect("Failed to create LoRa object");
    let config = lora.get_configuration().expect("Failed to fetch configuration");

    config.print_parameters();

    let new_config = Configuration::from_u8(
        config.command(),
        config.starting_address(),
        config.length(),

        0x00,
        0x03,

        Speed::from_u8(
            AirDataRate::AirDataRate010_24 as u8,
            UartParity::Mode00_8N1 as u8,
            UartBpsType::UartBps9600 as u8
        ),

        Opt::from_u8(
            TransmissionPower::Power22 as u8,
            RSSIAmbientNoiseEnable::Disabled as u8,
            SubPacketSetting::SPS200_00 as u8
        ),

        39,

        TransmissionMode::from_u8(
            WORPeriod::WOR2000_011 as u8,
            LBTEnable::Disabled as u8,
            FixedTransmission::Disabled as u8,
            RSSIEnable::Disabled as u8
        ),

        Crypt::new()
    );

    lora.set_configuration(new_config, true).expect("Failed to set configuration");

    let config = lora.get_configuration().expect("Failed to fetch configuration");
    config.print_parameters();
}