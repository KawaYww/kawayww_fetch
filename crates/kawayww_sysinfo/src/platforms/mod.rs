cfg_if::cfg_if! {
    if #[cfg(unix)] {
        pub mod unix;
        pub use unix as sys;
    } else if #[cfg(windows)] {
        pub mod windows;
        pub use windows as sys;
    }
}

pub struct Platform;

impl Platform {
    pub fn cpu_info(&self) -> Option<crate::models::CPUInfo> {
        sys::cpu_info::new()

    }
    pub fn uptime(&self) -> Option<crate::models::Uptime> {
        sys::uptime::new()
    }
}
