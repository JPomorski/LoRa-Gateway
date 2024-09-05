use lora_lib::lora::LoRa;

fn main() {
    let m0_pin = 0;
    let m1_pin = 0;

    let mut lora = LoRa::new(m0_pin, m1_pin).expect("Failed to create LoRa object");
    let config = lora.get_configuration().expect("Failed to fetch configuration");

    config.print_parameters();
}