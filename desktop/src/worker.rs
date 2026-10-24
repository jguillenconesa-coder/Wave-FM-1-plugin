// MIT. Scheduling applies only to the emulation worker owned by this instance.
// No process/global priorities, power plan, system timer or driver is changed.

#[cfg(windows)]
#[link(name = "avrt")]
unsafe extern "system" {
    fn AvSetMmThreadCharacteristicsW(task: *const u16, index: *mut u32) -> *mut std::ffi::c_void;
    fn AvRevertMmThreadCharacteristics(handle: *mut std::ffi::c_void) -> i32;
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn pthread_set_qos_class_self_np(class: u32, relative_priority: i32) -> i32;
}

pub struct Scheduling {
    #[cfg(windows)]
    handle: *mut std::ffi::c_void,
    pub status: u32,
}

impl Scheduling {
    pub fn audio() -> Self {
        #[cfg(windows)]
        {
            let task = [80u16, 114, 111, 32, 65, 117, 100, 105, 111, 0]; // Pro Audio
            let mut index = 0;
            // SAFETY: nul-terminated UTF-16 and a writable index live across the
            // call. The returned MMCSS registration belongs to this thread and
            // is reverted by Drop on the same thread. Failure is non-fatal.
            // https://learn.microsoft.com/windows/win32/api/avrt/nf-avrt-avsetmmthreadcharacteristicsw
            let handle = unsafe { AvSetMmThreadCharacteristicsW(task.as_ptr(), &mut index) };
            Self { handle, status: if handle.is_null() { 0 } else { 1 } }
        }
        #[cfg(target_os = "macos")]
        {
            // Public libSystem API since macOS 10.10; no realtime privilege is
            // requested. This changes only the current worker's QoS.
            let result = unsafe { pthread_set_qos_class_self_np(0x21, 0) };
            Self { status: if result == 0 { 2 } else { 0 } }
        }
        #[cfg(not(any(windows, target_os = "macos")))]
        Self { status: 0 }
    }
}

impl Drop for Scheduling {
    fn drop(&mut self) {
        #[cfg(windows)]
        if !self.handle.is_null() {
            // SAFETY: the successful registration has not been reverted and
            // this guard never leaves the worker thread that created it.
            unsafe { AvRevertMmThreadCharacteristics(self.handle); }
        }
        #[cfg(target_os = "macos")]
        {
            // The guard lives for the worker's whole lifetime; thread exit
            // releases its QoS. Do not alter the host thread's QoS.
        }
    }
}
