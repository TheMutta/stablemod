#[cfg(target_os = "linux")]
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

#[cfg(target_os = "linux")]
struct HalLogger;

#[cfg(target_os = "linux")]
impl log::Log for HalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}


#[cfg(target_os = "linux")]
static HAL_LOGGER: HalLogger = HalLogger;

#[cfg(target_os = "linux")]
pub fn init() {
    log::set_logger(&HAL_LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info)).unwrap()
}
