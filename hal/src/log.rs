use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

pub struct HalLogger;

#[cfg(all(target_os = "none", target_arch = "x86_64"))]
struct PortWriter;

#[cfg(target_os = "linux")]
struct SyscallWriter;

#[cfg(all(target_os = "none", target_arch = "x86_64"))]
impl PortWriter {
    fn outb(port: u16, value: u8) {
        use core::arch::asm;
        unsafe {
            asm!(
                "out dx, al",
                in("dx") port,
                in("al") value,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}


use core::fmt::Write;

#[cfg(all(target_os = "none", target_arch = "x86_64"))]
impl Write for PortWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        const COM1: u16 = 0x3F8; 

        for ch in s.bytes() {
            Self::outb(COM1, ch);
        }

        Ok(())
    }
}


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

impl log::Log for HalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            #[cfg(all(target_os = "none", target_arch = "x86_64"))]
            {
                let mut writer = PortWriter;
                let _ = write!(writer, "{} - {}\n", record.level(), record.args());
            }

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
