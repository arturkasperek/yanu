use eyre::{bail, Result};
use std::{
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
};
use tracing::{debug, info};

#[cfg(target_os = "windows")]
use crate::defines::{HACPACK, HACTOOL};
use crate::{defines::APP_CACHE_DIR, utils::move_file};

#[derive(Debug, Clone, Copy)]
pub enum Cache {
    Hacpack,
    Hactool,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Hactoolnet,
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(target_os = "windows")]
            Cache::Hacpack => write!(f, "hacpack.exe"),
            #[cfg(target_os = "windows")]
            Cache::Hactool => write!(f, "hactool.exe"),
            #[cfg(target_os = "windows")]
            Cache::Hactoolnet => write!(f, "hactoolnet.exe"),
            #[cfg(any(target_os = "linux", target_os = "android"))]
            Cache::Hacpack => write!(f, "hacpack"),
            #[cfg(any(target_os = "linux", target_os = "android"))]
            Cache::Hactool => write!(f, "hactool"),
            #[cfg(target_os = "linux")]
            Cache::Hactoolnet => write!(f, "hactoolnet"),
        }
    }
}

impl Cache {
    /// Saves the given file as a cache for `self`.
    ///
    /// Overwrites the previous cache in the process if any.
    pub fn from<P: AsRef<Path>>(self, path: P) -> Result<Self> {
        info!(?self, "Caching {:?}", path.as_ref());

        let cache_dir = APP_CACHE_DIR.as_path();
        fs::create_dir_all(cache_dir)?;
        let dest = cache_dir.join(self.to_string());
        if path.as_ref() != dest {
            move_file(path.as_ref(), dest)?;
        }

        Ok(self)
    }
    /// Extracts the embedded files to the cache dir
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    pub fn from_embed(self) -> Result<Self> {
        info!(?self, "Caching from embed");

        let cache_dir = APP_CACHE_DIR.as_path();
        fs::create_dir_all(cache_dir)?;
        let mut file = fs::File::create(cache_dir.join(self.to_string()))?;
        file.write_all(self.as_bytes())?;

        Ok(self)
    }
    /// Returns the path to the embedded resource.
    ///
    /// Cache is used if it exists else the embedded data is written to a file
    /// and the path is returned.
    pub fn path(&self) -> Result<PathBuf> {
        let cache_dir = APP_CACHE_DIR.as_path();
        fs::create_dir_all(cache_dir)?;

        let file_name = self.to_string();
        for entry in fs::read_dir(cache_dir)? {
            let entry = entry?;
            if entry.file_name().to_string_lossy() == file_name {
                // return cache if exists
                return Ok(entry.path());
            }
        }

        bail!("Failed to find {:?} in cache", self);
    }
    /// chmod +x
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn make_executable(self) -> Result<Self> {
        use std::process::Command;

        let cache_dir = APP_CACHE_DIR.as_path();
        fs::create_dir_all(cache_dir)?;

        let file_path = cache_dir.join(self.to_string());
        if self.is_cached() {
            if Command::new("chmod")
                .arg("+x")
                .arg(&file_path)
                .status()?
                .success()
            {
                info!(?file_path, "Given executable permission");
                return Ok(self);
            }
        }

        bail!("Failed to give executable permission to {:?}", file_path);
    }
    pub fn is_cached(&self) -> bool {
        if self._exists().is_ok() {
            return true;
        }
        false
    }
    fn _exists(&self) -> Result<()> {
        let cache_dir = APP_CACHE_DIR.as_path();
        fs::create_dir_all(cache_dir)?;

        let file_name = self.to_string();
        for entry in fs::read_dir(cache_dir)? {
            let entry = entry?;
            if entry.file_name().to_string_lossy() == file_name {
                return Ok(());
            }
        }

        bail!("{:?} isn't cached", file_name);
    }
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    fn as_bytes(&self) -> &'static [u8] {
        use crate::defines::HACTOOLNET;

        match self {
            #[cfg(target_os = "windows")]
            Cache::Hacpack => HACPACK,
            #[cfg(target_os = "windows")]
            Cache::Hactool => HACTOOL,
            Cache::Hactoolnet => HACTOOLNET,
            _ => unreachable!(),
        }
    }
}
