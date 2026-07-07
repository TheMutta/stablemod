#![no_std]
#![no_main]
#![feature(c_size_t)]
#![feature(c_variadic)]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod c_abi {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));

    use core::ffi::{c_int, c_void, c_size_t};
    use core::ptr::null_mut;

    #[link(name = "rump", kind = "static")]
    unsafe extern "C" {
        /// Bootstraps the Rump kernel components.
        /// Returns 0 on success, or a NetBSD errno value on failure.
        pub safe fn rump_init() -> c_int;
    }
}

extern crate alloc;

use core::{alloc::{GlobalAlloc, Layout}, panic::PanicInfo, ptr::null_mut};

struct BaseAlloc;

unsafe impl GlobalAlloc for BaseAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {

    }
}

#[global_allocator]
static mut GLOBAL_ALLOC: BaseAlloc = BaseAlloc;

use hal::syscall;
use hal::log::HalLogger;

use kernel::CapabilityHandle;
use kernel::ResourceCapability;
use kernel::ResourceCapabilityArena;

#[unsafe(no_mangle)]
extern "C" fn _start(arg0: u64, arg1: u64) {
    HalLogger::init();

    log::info!("Hello, world!");
    log::info!("Butler is alive!");

    let arena_handle = CapabilityHandle::new(arg0, arg1);
    let cap_handle = CapabilityHandle::new(arg0, arg1);

    let buf = [0u8; 4096];

    log::info!("Read!");
    let res = syscall!(
        kernel::SYS_CAP_READ,
        &arena_handle as *const CapabilityHandle as usize,
        &cap_handle as *const CapabilityHandle as usize,
        0,
        &buf as *const u8 as *mut u8 as usize,
        128
    );

    if res == 0 {
        log::info!("Valid read!");

        let arena = unsafe { core::mem::transmute::<[u8; 4096], ResourceCapabilityArena>(buf) };

        if arena.arenaid == arg1 {
            log::info!("read arena id and real arena id match!");
            log::info!("arena size: {}", arena.arena_cap.size);
            log::info!("arena: {:#?}", arena);

            let rescap = ResourceCapability::default();

            for idx in 0..(arena.slots - arena.slots_free) {
                let res = syscall!(
                    kernel::SYS_CAP_READ,
                    &arena_handle as *const CapabilityHandle as usize,
                    &cap_handle as *const CapabilityHandle as usize,
                    idx as usize * 64 + core::mem::offset_of!(ResourceCapabilityArena, res_cap) as usize,
                    &rescap as *const _ as *const u8 as *mut u8 as usize,
                    64
                );

                if res != 0 { log::info!("PANIC"); loop {} }

                log::info!("cap: {:#?}", rescap);
            }

        }
    }
                
    log::info!("Running rump_init()");
    crate::c_abi::rump_init();

    log::info!("Done!");

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
use core::ffi::{c_char, c_int, c_void, c_size_t};

// =========================================================================
// 1. INITIALIZATION & SYSTEM CORE
// =========================================================================

/// Initializes the hypercall layer. Called early during rump_init().
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_init(_version: c_int, _hyp: *const c_void) -> c_int {
    -1
}

/// Retrieves configuration parameters (like memory sizes or environment options).
/// Return -1 if the parameter is not set.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_getparam(_name: *const c_char, _buf: *mut c_void, _len: c_size_t) -> c_int {
    -1
}

/// Emergency exit. Called when the Rump Kernel panics.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_exit(_value: c_int) {
    loop {}
}

/// Low-level console printing for kernel diagnostic messages (printf output).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_putchar(_c: c_int) {}

/// Formatted string printing, used for kernel panic strings and debugging.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_dprintf(_fmt: *const c_char, _: ...) {}

// =========================================================================
// 2. MEMORY MANAGEMENT
// =========================================================================

/// Allocates page-aligned or specifically aligned virtual memory for the kernel heap.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_malloc(_size: c_size_t, _alignment: c_size_t) -> *mut c_void {
    core::ptr::null_mut()
}

/// Frees memory allocated by rumpuser_malloc.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_free(_ptr: *mut c_void, _size: c_size_t) {}

/// Maps a region of memory (similar to mmap). Used for backing virtual memory layout.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_anonmmap(_prefaddr: *mut c_void, _size: c_size_t, _align: c_size_t, _exec: c_int) -> *mut c_void {
    core::ptr::null_mut()
}

/// Unmaps memory regions mapped by anonmmap.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_unmap(_ptr: *mut c_void, _size: c_size_t) {}

// =========================================================================
// 3. MUTUAL EXCLUSION (MUTEXES)
// =========================================================================

/// Allocates and initializes a native mutex.
/// `cookie` specifies lock properties (like spinlock vs adaptive lock).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_mutex_init(_mtx: *mut *mut c_void, _cookie: c_int) {}

/// Acquires the mutex, blocking the current execution context until available.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_mutex_enter(_mtx: *mut c_void) {}

/// Acquires the mutex but promises not to perform any context-switch wraps.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_mutex_enter_nowrap(_mtx: *mut c_void) {}

/// Non-blocking attempt to lock a mutex. Returns 1 on success, 0 on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_mutex_tryenter(_mtx: *mut c_void) -> c_int {
    1
}

/// Releases a held mutex.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_mutex_exit(_mtx: *mut c_void) {}

/// Destroys the mutex and reclaims its underlying handle memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_mutex_destroy(_mtx: *mut c_void) {}

/// Returns the thread handle currently owning the mutex.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_mutex_owner(_mtx: *mut c_void) -> *mut c_void {
    core::ptr::null_mut()
}

// =========================================================================
// 4. CONDITION VARIABLES (CV)
// =========================================================================

/// Allocates and initializes an opaque condition variable context.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_cv_init(_cv: *mut *mut c_void) {}

/// Destroys the condition variable context.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_cv_destroy(_cv: *mut c_void) {}

/// Atomically releases the mutex and blocks until the condition variable is signaled.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_cv_wait(_cv: *mut c_void, _mtx: *mut c_void) {}

/// Identical to cv_wait, but ensures no additional scheduler wrapping layers occur.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_cv_wait_nowrap(_cv: *mut c_void, _mtx: *mut c_void) {}

/// Blocks with a maximum nanosecond time boundary. Returns 0 on signal, or `EWOULDBLOCK`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_cv_timedwait(_cv: *mut c_void, _mtx: *mut c_void, _sec: i64, _nsec: i64) -> c_int {
    0
}

/// Unblocks one thread waiting on the condition variable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_cv_signal(_cv: *mut c_void) {}

/// Unblocks all threads currently waiting on the condition variable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_cv_broadcast(_cv: *mut c_void) {}

/// Checks if there are any threads explicitly sleeping on this variable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_cv_has_waiters(_cv: *mut c_void) -> c_int {
    0
}

// =========================================================================
// 5. READ/WRITE LOCKS
// =========================================================================

/// Allocates and initializes an opaque Read/Write lock.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_rw_init(_rw: *mut *mut c_void) {}

/// Destroys the Read/Write lock.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_rw_destroy(_rw: *mut c_void) {}

/// Acquires the lock for reading or writing based on operations flag.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_rw_enter(_rw: *mut c_void, _op: c_int) {}

/// Attempts to acquire the lock. Returns 1 on success, 0 on blocking condition.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_rw_tryenter(_rw: *mut c_void, _op: c_int) -> c_int {
    1
}

/// Releases the Read/Write lock.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_rw_exit(_rw: *mut c_void) {}

/// Upgrades a read lock to a write lock on the fly.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_rw_tryupgrade(_rw: *mut c_void) -> c_int {
    1
}

/// Downgrades a write lock down to a read lock.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_rw_downgrade(_rw: *mut c_void) {}

/// Diagnostics check to verify if the lock is held.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_rw_held(_rw: *mut c_void, _op: c_int) -> c_int {
    1
}

// =========================================================================
// 6. THREADING & SCHEDULING
// =========================================================================

/// Spawns a new native execution thread context executing the function `f`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_thread_create(
    _f: extern "C" fn(*mut c_void, _: ...),
    _arg: *mut c_void,
    _name: *const c_char,
    _joinable: c_int,
    _priority: c_int,
    _cpuid: c_int,
    _thread_handle: *mut *mut c_void,
) -> c_int {
    0
}

/// Causes the active thread context to terminate.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_thread_exit() {
    loop {}
}

/// Blocks the current thread until the target thread handle finishes executing.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_thread_join(_thread: *mut c_void) {}

/// Returns the current thread context pointer representation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_thread_self() -> *mut c_void {
    core::ptr::null_mut()
}

/// Yields control back to your microkernel scheduler.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_thread_yield() {}

// =========================================================================
// 7. CLOCKS & TIMEKEEPING
// =========================================================================

/// Fetches current real-world or monotonic system clock time matching NetBSD ids.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_clock_gettime(_clock_id: c_int, _sec: *mut i64, _nsec: *mut i64) -> c_int {
    0
}

/// Suspends execution of the calling context thread for a discrete timeframe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_clock_sleep(_clock_id: c_int, _sec: i64, _nsec: i64) -> c_int {
    0
}

// =========================================================================
// 8. STORAGE & FILE I/O (OPTIONAL VFS OVERRIDES)
// =========================================================================

/// Opens a backing host file or raw storage partition capability.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_open(_path: *const c_char, _flags: c_int, _fd: *mut c_int) -> c_int {
    -1
}

/// Closes a backing storage description capability.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_close(_fd: c_int) -> c_int {
    -1
}

/// Vectorized scattered reads from storage descriptors.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_iovread(_fd: c_int, _iov: *mut c_void, _iovcnt: c_size_t, _off: i64, _retval: *mut c_size_t) -> c_int {
    -1
}

/// Vectorized gathered writes to storage descriptors.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_iovwrite(_fd: c_int, _iov: *const c_void, _iovcnt: c_size_t, _off: i64, _retval: *mut c_size_t) -> c_int {
    -1
}

/// Synchronizes buffered modifications down to the storage media.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_fsync(_fd: c_int) -> c_int {
    -1
}

// =========================================================================
// 9. DYNAMIC LINKING & MODULE ENHANCEMENTS
// =========================================================================

/// Dynamic linker bootstrap hook.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_dl_bootstrap(_elf_base: *mut c_void, _size: c_size_t) -> c_int {
    0
}


/// Handles daemon fork orchestration. Safe to stub as 0 for your microkernel.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_daemonize_begin() -> c_int {
    0
}

/// Concludes daemon orchestration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_daemonize_done(_error: c_int) {}

/// Sets or switches the current Lightweight Process (thread pointer storage context).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_curlwpop(_op: c_int, _lwp: *mut c_void) {}

/// Returns the active current thread pointer storage context back to Rump.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_curlwp() -> *mut c_void {
    core::ptr::null_mut()
}

/// Routes an internal abort or diagnostic signal down to your microkernel thread.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_kill(_pid: c_int, _sig: c_int) -> c_int {
    0
}

/// populates target buffers with entropy seeds. Map to your hardware RNG / kernel source.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rumpuser_getrandom(_buf: *mut c_void, _len: c_size_t, _flags: c_int, _retval: *mut c_size_t) -> c_int {
    0
}
