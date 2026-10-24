// MIT. Scheduling applies only to the emulation worker owned by this instance.
// No process/global priorities, power plan, system timer or driver is changed.

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentThread() -> *mut std::ffi::c_void;
    fn GetThreadPriority(thread: *mut std::ffi::c_void) -> i32;
    fn SetThreadPriority(thread: *mut std::ffi::c_void, priority: i32) -> i32;
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn pthread_set_qos_class_self_np(class: u32, relative_priority: i32) -> i32;
}

pub struct Scheduling {
    #[cfg(windows)]
    handle: *mut std::ffi::c_void,
    #[cfg(windows)]
    previous: Option<i32>,
    pub status: u32,
}

impl Scheduling {
    pub fn audio() -> Self {
        #[cfg(windows)]
        {
            // Above normal (+1), not a realtime priority or an MMCSS task.
            // Remain below ordinary highest/time-critical host audio threads;
            // avoid MMCSS's CPU quotas for this asynchronous background worker.
            // Never lower an existing higher priority, including a test host.
            // SAFETY: the pseudo-handle belongs to this thread and stays here;
            // this non-Send guard restores a successful change on the same
            // thread. There are no pointer dereferences or owned OS handles.
            // https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority
            let handle = unsafe { GetCurrentThread() };
            let previous = unsafe { GetThreadPriority(handle) };
            if previous == i32::MAX { return Self { handle, previous: None, status: 0 }; }
            if previous >= 1 { return Self { handle, previous: None, status: 1 }; }
            let changed = unsafe { SetThreadPriority(handle, 1) } != 0;
            Self { handle, previous: changed.then_some(previous), status: u32::from(changed) }
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
        if let Some(previous) = self.previous {
            // SAFETY: this guard stays on its creating thread; its pseudo-handle
            // remains valid until thread exit. Restore only our own change.
            unsafe { SetThreadPriority(self.handle, previous); }
        }
        #[cfg(target_os = "macos")]
        {
            // The guard lives for the worker's whole lifetime; thread exit
            // releases its QoS. Do not alter the host thread's QoS.
        }
    }
}
