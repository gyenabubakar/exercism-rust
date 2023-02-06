#![allow(unused)]

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Debug,
    Error,
}

pub fn log(level: LogLevel, message: &str) -> String {
    let level_str = format!("{:?}", level).to_uppercase();
    format!("[{level_str}]: {message}")
}

pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}

pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}

pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
