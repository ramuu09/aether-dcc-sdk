use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard};

const HIGH_PRIORITY_CLASS: u32 = 0x0000_0080;

#[link(name = "kernel32", kind = "raw-dylib")]
extern "system" {
    fn GetCurrentProcess() -> *mut c_void;
    fn SetPriorityClass(process: *mut c_void, class: u32) -> i32;
}

pub fn cpu_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|poison| poison.into_inner())
}

pub fn elevate() {
    static DONE: AtomicU32 = AtomicU32::new(0);
    if DONE.swap(1, Ordering::Relaxed) != 0 {
        return;
    }
    unsafe {
        SetPriorityClass(GetCurrentProcess(), HIGH_PRIORITY_CLASS);
    }
}
