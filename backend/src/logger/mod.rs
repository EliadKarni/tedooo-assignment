use std::sync::Once;
use std::env;
use log::{LevelFilter, Metadata, Record, SetLoggerError, set_boxed_logger, set_max_level};

pub struct Logger {
    level: LevelFilter,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static INIT: Once = Once::new();

pub fn init_logger() -> Result<(), SetLoggerError> {
    let level = match env::var("LOG_LEVEL") {
        Ok(val) => val.parse().unwrap_or(LevelFilter::Info),
        Err(_) => LevelFilter::Info,
    };
    INIT.call_once(|| {
        let logger = Box::new(Logger { level });
        set_max_level(level);
        let _ = set_boxed_logger(logger);
    });
    Ok(())
}
