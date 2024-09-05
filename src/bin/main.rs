use std::mem::size_of;
use lora_lib::enums::packet_length::PacketLength;
use lora_lib::lora::LoRa;
use lora_lib::utility::configuration::Configuration;

fn main() {
    println!("{}", size_of::<Configuration>());
    println!(
        "{} + {} + {} + {}",
        PacketLength::Configuration.length(),
        PacketLength::Sped.length(),
        PacketLength::Option.length(),
        PacketLength::TransmissionMode.length()
    );

    let _lora = LoRa::new(1, 1).expect("oh no");
}