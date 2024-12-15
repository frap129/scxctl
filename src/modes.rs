use clap::ValueEnum;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Auto,
    Gaming,
    Powersave,
    Lowlatency,
    Server,
    Unknown,
}
impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Auto => "auto",
            Mode::Gaming => "gaming",
            Mode::Powersave => "powersave",
            Mode::Lowlatency => "lowlatency",
            Mode::Server => "server",
            Mode::Unknown => "unknown",
        }
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            Mode::Auto => 0,
            Mode::Gaming => 1,
            Mode::Powersave => 2,
            Mode::Lowlatency => 3,
            Mode::Server => 4,
            Mode::Unknown => 5,
        }
    }

    pub fn from_u32(u: u32) -> Self {
        match u {
            0 => Mode::Auto,
            1 => Mode::Gaming,
            2 => Mode::Powersave,
            3 => Mode::Lowlatency,
            4 => Mode::Server,
            _ => Mode::Unknown,
        }
    }
}
