use std::{fmt::Display, sync::Mutex};

use once_cell::sync::OnceCell;
use utils::{show_color, Color};

static LOGGER: OnceCell<Box<dyn Log + Send + Sync>> = OnceCell::new();

#[derive(PartialEq, Eq, PartialOrd)]
pub enum Level {
    Error,
    Warn,
    Info,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Error => { 
            write!(
                f,
                "{}",
                show_color("Error", Color::RED)
            )},
            Level::Warn => { 
            write!(
                f,
                "{}",
                show_color("Warning", Color::BLUE)
            )},
            Level::Info => { 
                write!(
                    f,
                    "{}",
                    "Info"
                )},
        }
    }
}


struct SimpleLogger {
    max_level: Level,
}


impl Log for SimpleLogger {
    fn enabled(&self, level: &Level) -> bool {
        *level <= self.max_level
    }

    fn log(&self, level: &Level, message: &str) {
        println!("[{}] {}", level, message);
    }
}

// Trait implement
pub trait Log {
    fn enabled(&self, level: &Level) -> bool;
    fn log(&self, level: &Level, message: &str);
}

// invoke 
fn log(level: Level, message: &str) {
    if let Some(logger) = LOGGER.get() {
        if logger.enabled(&level) {
            logger.log(&level, message);
        }
    }
}

pub fn error(message: &str) {
    log(Level::Error, message);
}

pub fn warn(message: &str) {
    log(Level::Warn, message);
}

pub fn info(message: &str) {    
    log(Level::Info, message);
}


#[derive(Debug)]
pub struct SetLoggerError;

pub fn set_boxed_logger(logger: Box<dyn Log + Sync + Send>) -> Result<(), SetLoggerError> {
    if LOGGER.set(logger).is_err() {
        return Err(SetLoggerError);
    }

    Ok(())
}

pub fn init_log(max_level: Level) {
    set_boxed_logger(Box::new(SimpleLogger { max_level }))
        .expect("Logger has been already set");
}
