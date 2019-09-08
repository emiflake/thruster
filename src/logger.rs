use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use termion::color;

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{}] [{}{}{}] {}",
                chrono::Utc::now().format("%H:%M:%S"),
                color::Fg(color::Cyan),
                record.level(),
                color::Fg(color::Reset),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
