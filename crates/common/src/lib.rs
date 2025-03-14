#[cfg(not(any(
    all(
        target_arch = "x86_64",
        any(windows, unix),
        not(feature = "android-proot")
    ),
    all(target_arch = "aarch64", feature = "android-proot")
)))]
compile_error!("This traget configuration is not supported");

pub mod defines;
pub mod filename;
pub mod utils;

#[cfg(target_os = "android")]
pub mod termux;
