use std::path::PathBuf;

pub const APP_DIR: &str = "com.github.nozwock.yanu";

#[cfg(target_os = "linux")]
pub const HACPACK: &[u8] = include_bytes!("../resources/x86_64-linux/hacpack");
#[cfg(target_os = "linux")]
pub const HACTOOL: &[u8] = include_bytes!("../resources/x86_64-linux/hactool");

#[cfg(target_os = "windows")]
pub const HACPACK: &[u8] = include_bytes!("../resources/x86_64-windows/hacpack.exe");
#[cfg(target_os = "windows")]
pub const HACTOOL: &[u8] = include_bytes!("../resources/x86_64-windows/hactool.exe");

#[cfg(target_os = "android")]
pub const HACPACK: &[u8] = include_bytes!("../resources/aarch64-android/hacpack");
#[cfg(target_os = "android")]
pub const HACTOOL: &[u8] = include_bytes!("../resources/aarch64-android/hactool");

pub fn app_cache_dir() -> PathBuf {
    dirs::cache_dir().unwrap_or_default().join(APP_DIR)
}
