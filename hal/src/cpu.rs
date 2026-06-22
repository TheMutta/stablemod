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
    pub fn new() -> Self {
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

        unsafe { Efer::write(Efer::read().union(EferFlags::SYSTEM_CALL_EXTENSIONS)); }
        Star::write(user_code_segment, user_data_segment, code_segment,data_segment).unwrap();

        core::hint::black_box(syscall_entry as unsafe extern "C" fn());
        LStar::write(VirtAddr::new(syscall_entry as *const () as u64));

        interrupts::enable();
    }
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn divide_error_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn debug_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn non_maskable_interrupt_handler(_isf: InterruptStackFrame) {
    loop {}
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
extern "x86-interrupt" fn breakpoint_handler(_isf: InterruptStackFrame) {
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

use core::arch::naked_asm;

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".syscall_trampoline")]
unsafe extern "C" fn syscall_entry() {
     naked_asm!(
         "sysretq",
         );
}
