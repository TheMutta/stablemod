pub struct HalRngGenerator;

impl HalRngGenerator {
    pub fn new() -> Self { Self {} }
    pub fn generate_rng(&self, buf: &mut [u64]) {
        #[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os = "uefi")))]
        unsafe {
            for val in buf {
                let mut random_val: u64;
                core::arch::asm!(
                    "2:",
                    "rdrand {}",
                    "jae 2b",
                    out(reg) random_val
                );
                *val = random_val;
            }
        }
    }
}
