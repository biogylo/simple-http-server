use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::Context;

pub struct LoadedFile {
    pub filepath: PathBuf,
    pub last_modified: SystemTime,
    pub contents: Vec<u8>,
}

pub fn check_last_modified(filepath: &Path) -> anyhow::Result<SystemTime> {
    fs::metadata(filepath)
        .and_then(|md| md.modified())
        .map_err(anyhow::Error::from)
        .context(format!("Unable to metadata from {:?}", &filepath))
}

impl LoadedFile {
    pub fn try_reload(self) -> Result<LoadedFile, (LoadedFile, anyhow::Error)> {
        let Ok(last_mtime) = check_last_modified(&self.filepath) else {
            let err_str = format!(
                "Didn't reload {:?}, since we were unable to get last load time",
                self.filepath
            );
            return Err((self, anyhow::Error::msg(err_str)));
        };
        if SystemTime::now() > last_mtime {
            return Ok(self);
        }
        let maybe_file = LoadedFile::from_path(&self.filepath);
        if let Ok(file) = maybe_file {
            Ok(file)
        } else {
            let err = format!(
                "Didn't reload {:?}, since we were unable to load its contents",
                self.filepath
            );
            Err((self, anyhow::Error::msg(err)))
        }
    }
    pub fn from_path(filepath: &Path) -> anyhow::Result<LoadedFile> {
        let contents = fs::read(filepath)
            .map_err(anyhow::Error::from)
            .context(format!("Unable to read from {:?}", &filepath))?;
        let last_modified = check_last_modified(filepath)?;
        Ok(LoadedFile {
            filepath: filepath.to_path_buf(),
            last_modified,
            contents,
        })
    }
}
