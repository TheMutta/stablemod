#[cfg(target_os = "linux")]
pub struct HalProcessor {
}

#[cfg(target_os = "linux")]
impl HalProcessor {
    pub const fn new() -> Self {
        Self { }
    }
    pub fn init(&mut self) {
    }
}
 

#[cfg(target_arch = "aarch64")]
pub struct HalProcessor {
}

#[cfg(target_arch = "aarch64")]
use aarch64_cpu::registers::Writeable;

#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.exceptions")]
pub unsafe extern "C" fn exception_vector_table() {
    core::arch::naked_asm!(
        ".balign 2048",
        // Current EL with SP0 (Synchronous, IRQ, FIQ, SError)
        ".balign 0x80", "b exception_handler",
        ".balign 0x80", "b exception_handler",
        ".balign 0x80", "b exception_handler",
        ".balign 0x80", "b exception_handler",

        // Current EL with SPx
        ".balign 0x80", "b exception_handler",
        ".balign 0x80", "b exception_handler",
        ".balign 0x80", "b exception_handler",
        ".balign 0x80", "b exception_handler",
    );
}

#[unsafe(no_mangle)]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn exception_handler() {
    // Your panic/handling logic here
    loop {}
}

#[cfg(target_arch = "aarch64")]
impl HalProcessor {
    pub unsafe fn init_exceptions() {
        use aarch64_cpu::registers::VBAR_EL1;

        core::hint::black_box(exception_vector_table as unsafe extern "C" fn());

        // Set the Vector Base Address Register (VBAR_EL1)
        VBAR_EL1.set(exception_vector_table as usize as u64);
        core::arch::asm!("isb");
    }

    pub const fn new() -> Self {
        Self {

        }
    }

    pub fn init(&mut self) {
        use aarch64_cpu::registers::{CPACR_EL1, SP_EL1, SPSel};
        CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
    }

}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
use x86_64::{
    addr::VirtAddr,
    structures::{
        gdt::{
            Descriptor,
            GlobalDescriptorTable
        },
        tss::TaskStateSegment,
        idt::{
            InterruptDescriptorTable,
            InterruptStackFrame,
            PageFaultErrorCode
        },
    },
    registers::{
        segmentation::{
            Segment,
            CS,
            DS,
            ES,
            FS,
            GS,
            SS
        },
        model_specific::{
            Star,
            Efer,
            EferFlags,
            LStar
        },
    },
    instructions::{
        tables::load_tss,
        interrupts
    },
};

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
pub struct HalProcessor {
    gdt: GlobalDescriptorTable,
    tss: TaskStateSegment,
    idt: InterruptDescriptorTable,
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
impl HalProcessor {
    pub const fn new() -> Self {
        Self {
            gdt: GlobalDescriptorTable::new(),
            tss: TaskStateSegment::new(),
            idt: InterruptDescriptorTable::new(),
        }
    }

    pub fn init(&mut self) {
        interrupts::disable();

        let gdt = &mut self.gdt;
        let tss = &mut self.tss;
        let idt = &mut self.idt;

        let code_segment = gdt.append(Descriptor::kernel_code_segment());
        let data_segment = gdt.append(Descriptor::kernel_data_segment());
        let user_data_segment = gdt.append(Descriptor::user_data_segment());
        let user_code_segment = gdt.append(Descriptor::user_code_segment());
        let tss_segment = gdt.append(unsafe { Descriptor::tss_segment_unchecked(tss) });

        unsafe { gdt.load_unsafe(); }

        unsafe {
            CS::set_reg(code_segment);

            load_tss(tss_segment);

            DS::set_reg(data_segment);
            ES::set_reg(data_segment);
            FS::set_reg(data_segment);
            GS::set_reg(data_segment);
            SS::set_reg(data_segment);

        }

        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.cp_protection_exception.set_handler_fn(cp_protection_exception_handler);
        idt.hv_injection_exception.set_handler_fn(hv_injection_exception_handler);
        idt.vmm_communication_exception.set_handler_fn(vmm_communication_exception_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);

        unsafe {
            idt.load_unsafe();
        }

        unsafe extern "C" {
            fn syscall_vector_table();
        }
        unsafe { Efer::write(Efer::read().union(EferFlags::SYSTEM_CALL_EXTENSIONS)); }
        Star::write(user_code_segment, user_data_segment, code_segment,data_segment).unwrap();

        core::hint::black_box(syscall_vector_table as unsafe extern "C" fn());
        LStar::write(VirtAddr::new(syscall_vector_table as *const () as u64));

        interrupts::enable();
    }
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn divide_error_handler(_isf: InterruptStackFrame) {
    log::error!("divide error");
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn debug_handler(_isf: InterruptStackFrame) {
    log::error!("debug handler");
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn non_maskable_interrupt_handler(_isf: InterruptStackFrame) {
    log::error!("NMI");
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn breakpoint_handler(_isf: InterruptStackFrame) {
    log::error!("breakpoint handler");
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn overflow_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn bound_range_exceeded_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn invalid_opcode_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn device_not_available_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn double_fault_handler(_isf: InterruptStackFrame, _error_code: u64) -> ! {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn invalid_tss_handler(_isf: InterruptStackFrame, _error_code: u64) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn segment_not_present_handler(_isf: InterruptStackFrame, _error_code: u64) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn stack_segment_fault_handler(_isf: InterruptStackFrame, _error_code: u64) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn general_protection_fault_handler(_isf: InterruptStackFrame, _error_code: u64) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn page_fault_handler(_isf: InterruptStackFrame, _error_code: PageFaultErrorCode) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn x87_floating_point_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn alignment_check_handler(_isf: InterruptStackFrame, _error_code: u64) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn machine_check_handler(_isf: InterruptStackFrame) -> ! {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn simd_floating_point_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn virtualization_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn cp_protection_exception_handler(_isf: InterruptStackFrame, _error_code: u64) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn hv_injection_exception_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn vmm_communication_exception_handler(_isf: InterruptStackFrame, _error_code: u64) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn security_exception_handler(_isf: InterruptStackFrame, _error_code: u64) {
    loop {}
}

#[inline(always)]
pub unsafe fn do_userland_jump(userland_ip: u64, userland_sp: u64, arg0: u64, arg1: u64) {
    #[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
    unsafe {
        core::arch::asm!(
            //"mov rsp, rax",
	    "mov r11, 0x202",
            "sysretq",
            in("rdi") arg0,
            in("rsi") arg1,
            in("rax") userland_sp,
            in("rcx") userland_ip,
            options(noreturn),
        );
    }
}

#[inline(always)]
pub unsafe fn do_syscall(sys_num: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize) -> usize {
    let mut ret: usize = 0;

    #[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os = "uefi")))]
    unsafe {
        core::arch::asm!(
            "mov r10, rcx",
            "syscall",
            in("rax") sys_num,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("rcx") arg4,
            in("r8") arg5,
            in("r9") arg6,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
        );
    }

    ret
}

#[macro_export]
macro_rules! syscall {
    ($sys_num:expr) => {
        unsafe { $crate::cpu::do_syscall($sys_num, 0, 0, 0, 0, 0, 0) }
    };

    ($sys_num:expr, $arg1:expr) => {
        unsafe { $crate::cpu::do_syscall($sys_num, $arg1, 0, 0, 0, 0, 0) }
    };

    ($sys_num:expr, $arg1:expr, $arg2:expr) => {
        unsafe { $crate::cpu::do_syscall($sys_num, $arg1, $arg2, 0, 0, 0, 0) }
    };

    ($sys_num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        unsafe { $crate::cpu::do_syscall($sys_num, $arg1, $arg2, $arg3, 0, 0, 0) }
    };

    ($sys_num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        unsafe { $crate::cpu::do_syscall($sys_num, $arg1, $arg2, $arg3, $arg4, 0, 0) }
    };

    ($sys_num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        unsafe { $crate::cpu::do_syscall($sys_num, $arg1, $arg2, $arg3, $arg4, $arg5, 0) }
    };

    ($sys_num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        unsafe { $crate::cpu::do_syscall($sys_num, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6) }
    };
}

#[macro_export]
macro_rules! batch_syscalls {
    (
        $( $name:ident => $func:expr ),* $(,)?
    ) => {
        $(
            #[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os = "uefi")))]
            #[unsafe(no_mangle)]
            #[unsafe(naked)]
            #[unsafe(link_section = ".text.syscalls")]
            pub extern "C" fn $name() {
                core::arch::naked_asm!(
                    "jmp {}",
                    sym $func
                );
            }
        )*

        pub const NR_syscalls: usize = {
            let mut count = 0;
            $(
                let _ = stringify!($name);
                count += 1;
            )*
            count
        };

        #[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os = "uefi")))]
        #[unsafe(no_mangle)]
        #[unsafe(naked)]
        #[unsafe(link_section = ".text.syscalls")]
        pub unsafe extern "C" fn syscall_vector_table() {
            core::arch::naked_asm!(
                ".global syscall_table",
                "dispatch_syscall:",
                "   cmp rax, {NR_SYS}",
                "   jae .invalid_syscall",
                
                "   push rcx",
                "   push r11",

                "   lea r11, [rip + syscall_table]",
                "   movsxd rax, dword ptr [r11 + 4 * rax]",
                "   add rax, r11",

                "   mov rcx, r10", // 4th arg
                                   
                "   call rax",
                
                "   pop r11",
                "   pop rcx",
                "   sysretq",
                ".invalid_syscall:",
                "   mov rax, -1",
                "   sysretq",
                ".align 4",
                "syscall_table:",
                $(
                    concat!("   .long {", stringify!($name), "} - syscall_table")
                ),*
                ,
                NR_SYS = const NR_syscalls,
                $(
                    $name = sym $name
                ),*
            );
        }
    };
}
