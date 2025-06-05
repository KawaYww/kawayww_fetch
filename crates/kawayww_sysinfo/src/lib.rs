#![allow(unused)]

mod models;

cfg_if::cfg_if! {
    if #[cfg(any(
        target_os = "macos", target_os = "ios",
        target_os = "linux", target_os = "android",
        target_os = "freebsd"))
    ] {
       mod unix;
    }
}

pub use models::*;
