pub enum ProgramCommand {
    WriteCfgPwrDwnSave,
    ReadConfiguration,
    WriteCfgPwrDwnLose,
    WrongFormat,
    ReturnedCommand,
    SpecialWifiConfCommand
}

impl ProgramCommand {
    pub fn code(&self) -> u8 {
        match self {
            ProgramCommand::WriteCfgPwrDwnSave => 0xC0,
            ProgramCommand::ReadConfiguration => 0xC1,
            ProgramCommand::WriteCfgPwrDwnLose => 0xC2,
            ProgramCommand::WrongFormat => 0xFF,
            ProgramCommand::ReturnedCommand => 0xC1,
            ProgramCommand::SpecialWifiConfCommand => 0xCF
        }
    }
}

pub enum RegisterAddress {
    Configuration = 0x00,
    Sped = 0x02,
    TransmissionMode = 0x03,
    Channel = 0x04,
    Option = 0x05,
    Crypt = 0x06,
    Pid = 0x08
}

pub enum PacketLength {
    Configuration,
    Sped,
    Option,
    TransmissionMode,
    Channel,
    Crypt,
    Pid
}

impl PacketLength {
    pub fn length(&self) -> u8 {
        match self {
            PacketLength::Configuration => 0x08,
            PacketLength::Sped => 0x01,
            PacketLength::Option => 0x01,
            PacketLength::TransmissionMode => 0x01,
            PacketLength::Channel => 0x01,
            PacketLength::Crypt => 0x02,
            PacketLength::Pid => 0x03
        }
    }
}

#[repr(u8)]
pub enum AirDataRate {
    AirDataRate000_24 = 0b000,
    AirDataRate001_24 = 0b001,
    AirDataRate010_24 = 0b010,
    AirDataRate011_48 = 0b011,
    AirDataRate100_96 = 0b100,
    AirDataRate101_192 = 0b101,
    AirDataRate110_384 = 0b110,
    AirDataRate111_625 = 0b111
}

pub fn get_air_data_rate_description_by_params(byte: u8) -> String {
    let air_data_rate = from_byte(byte);

    if air_data_rate.is_none() {
        return "Invalid Air Data Rate!".to_string()
    }

    match air_data_rate.unwrap() {
        AirDataRate::AirDataRate000_24 => "2.4kbps".to_string(),
        AirDataRate::AirDataRate001_24 => "2.4kbps".to_string(),
        AirDataRate::AirDataRate010_24 => "2.4kbps (default)".to_string(),
        AirDataRate::AirDataRate011_48 => "4.8kbps".to_string(),
        AirDataRate::AirDataRate100_96 => "9.6kbps".to_string(),
        AirDataRate::AirDataRate101_192 => "19.2kbps".to_string(),
        AirDataRate::AirDataRate110_384 => "38.4kbps".to_string(),
        AirDataRate::AirDataRate111_625 => "62.5kbps".to_string(),
    }
}

pub fn from_byte(byte: u8) -> Option<AirDataRate> {
    match byte {
        0b000 => Some(AirDataRate::AirDataRate000_24),
        0b001 => Some(AirDataRate::AirDataRate001_24),
        0b010 => Some(AirDataRate::AirDataRate010_24),
        0b011 => Some(AirDataRate::AirDataRate011_48),
        0b100 => Some(AirDataRate::AirDataRate100_96),
        0b101 => Some(AirDataRate::AirDataRate101_192),
        0b110 => Some(AirDataRate::AirDataRate110_384),
        0b111 => Some(AirDataRate::AirDataRate111_625),
        _ => None
    }
}

#[derive(Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ModeType {
    Mode0 = 0,
    Mode1 = 1,
    Mode2 = 2,
    Mode3 = 3,
    ModeInit = 0xFF
}

impl ModeType {
    pub const MODE_0_NORMAL: Self = Self::Mode0;
    pub const MODE_0_TRANSMISSION: Self = Self::Mode0;

    pub const MODE_1_WOR_TRANSMITTER: Self = Self::Mode1;
    pub const MODE_1_WOR: Self = Self::Mode1;

    pub const MODE_2_WOR_RECEIVER: Self = Self::Mode2;
    pub const MODE_2_POWER_SAVING: Self = Self::Mode2;

    pub const MODE_3_CONFIGURATION: Self = Self::Mode3;
    pub const MODE_3_PROGRAM: Self = Self::Mode3;
    pub const MODE_3_SLEEP: Self = Self::Mode3;

    pub const MODE_INIT: Self = Self::ModeInit;
}

pub enum SubPacketSetting {
    SPS200_00 = 0b00,
    SPS128_01 = 0b01,
    SPS064_10 = 0b10,
    SPS032_11 = 0b11
}

pub enum RSSIAmbientNoiseEnable {
    Disabled = 0b0,
    Enabled = 0b1
}

pub enum WORPeriod {
    WOR500_000 = 0b000,
    WOR1000_001 = 0b001,
    WOR1500_010 = 0b010,
    WOR2000_011 = 0b011,
    WOR2500_100 = 0b100,
    WOR3000_101 = 0b101,
    WOR3500_110 = 0b110,
    WOR4000_111 = 0b111
}

pub enum LBTEnable {
    Disabled = 0b0,
    Enabled = 0b1
}

pub enum RSSIEnable {
    Disabled = 0b0,
    Enabled = 0b1
}

pub enum FixedTransmission {
    Disabled = 0b0,
    Enabled = 0b1
}

pub enum TransmissionPower {
    Power22 = 0b00,
    Power17 = 0b01,
    Power13 = 0b10,
    Power10 = 0b11
}
