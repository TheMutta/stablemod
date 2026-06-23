use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

#[cfg(target_os = "none")]
pub struct HalLogger;

#[cfg(target_os = "none")]
impl log::Log for HalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
        }
    }

    fn flush(&self) {}
}

#[cfg(any(target_os = "linux", target_os = "uefi"))]
pub struct HalLogger;

#[cfg(target_os = "linux")]
struct SyscallWriter;

use core::fmt::Write;

#[cfg(target_os = "linux")]
impl core::fmt::Write for SyscallWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Syscall 1 is write(fd, buf, len)
        // Using a crate like 'syscalls'
        let _ = unsafe { syscalls::syscall!(
            syscalls::Sysno::write,
            1,            // fd: stdout
            s.as_ptr(),   // buf
            s.len()       // len
        ) };
        Ok(())
    }
}

#[cfg(any(target_os = "linux", target_os = "uefi"))]
impl log::Log for HalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            #[cfg(target_os = "uefi")]
            {
                use uefi::println;

                println!("{} - {}", record.level(), record.args());
            }

            #[cfg(target_os = "linux")]
            {
                let mut writer = SyscallWriter;
                // This writes directly to the syscall, no intermediate String/vec
                let _ = write!(writer, "{} - {}\n", record.level(), record.args());
            }
        }
    }

    fn flush(&self) {}
}

static HAL_LOGGER: HalLogger = HalLogger;

impl HalLogger {
    pub fn init() {
        log::set_logger(&HAL_LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Info)).unwrap()
    }
}
