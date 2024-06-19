pub struct CalculationOption {
    pub disable_color_operations: bool,
}

impl CalculationOption {
    pub fn new(disable_color_operations: bool) -> CalculationOption {
        CalculationOption {
            disable_color_operations,
        }
    }
}

pub enum Level {
    Info,
    Warning,
    Error,
}

impl Level {
    pub fn colorize(self) -> String {
        match self {
            Self::Info => String::from(""),
            Self::Warning => String::from("\u{001b}[33m"),
            Self::Error => String::from("\u{001b}[31m"),
        }
    }
}

pub struct Terminal();

impl Terminal {
    pub fn cli_print(level: Level, msg: &str) {
        println!("{}{}\u{0001b}[0m", level.colorize(), msg);
    }   
}