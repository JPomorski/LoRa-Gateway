pub mod program_command {
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
}

pub mod mode_type {
    pub enum ModeType {

    }
}
