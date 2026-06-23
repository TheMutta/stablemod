pub trait HalHypervisorTrait {

}

pub struct HalKVMHypervisor {
    kvm_fd: i32,
}

#[repr(C)]
struct kvm_userspace_memory_region {
    slot: u32,
    flags: u32,
    guest_phys_addr: u64,
    memory_size: u64,
    userspace_addr: u64,
}


#[cfg(target_os = "linux")]
impl HalKVMHypervisor {
    pub fn init() {
        use alloc::vec;
        const O_RDWR: i32 = 2;
        let kvm_fd = unsafe {syscalls::syscall!(
            syscalls::Sysno::open,
            "/dev/kvm\0".as_ptr(),
            O_RDWR
        ).unwrap() };

        const KVM_CREATE_VM: i32 = 0x4020AE01;
        let vm_fd = unsafe { syscalls::syscall!(
            syscalls::Sysno::ioctl,
            kvm_fd,
            KVM_CREATE_VM,
            0
        ).unwrap() };

        let mut guest_ram = vec![0u8; 0x100000];
        let mem_slot = kvm_userspace_memory_region {
            slot: 0,
            guest_phys_addr: 0x0000_00000,
            flags: 0,
            memory_size: guest_ram.len() as u64,
            userspace_addr: guest_ram.as_mut_ptr() as u64,
        };

        const KVM_SET_USER_MEMORY_REGION: i32 = 0x4020AE46;
        let _ = unsafe { syscalls::syscall!(
            syscalls::Sysno::ioctl,
            vm_fd,
            KVM_SET_USER_MEMORY_REGION,
            &mem_slot as *const kvm_userspace_memory_region
        ) };

        const KVM_CREATE_VCPU: i32 = 0x4020AE41;
        let vcpu_fd = unsafe { syscalls::syscall!(
            syscalls::Sysno::ioctl,
            vm_fd,
            KVM_CREATE_VCPU,
            0
        ).unwrap() };
    }
}
