use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::Context;

pub struct LoadedFile {
    pub filepath: PathBuf,
    pub last_read: SystemTime,
    pub contents: Vec<u8>,
}

pub fn check_last_modified(filepath: &Path) -> anyhow::Result<SystemTime> {
    fs::metadata(filepath)
        .and_then(|md| md.modified())
        .map_err(anyhow::Error::from)
        .context(format!("Unable to metadata from {:?}", &filepath))
}

pub enum ReloadResult<'a> {
    NotNeeded(&'a LoadedFile),
    Reloaded(LoadedFile),
    ErrorDidntReload((&'a LoadedFile, String)),
}

impl LoadedFile {
    pub fn try_reload(&self) -> ReloadResult {
        let Ok(last_mtime) = check_last_modified(&self.filepath) else {
            return ReloadResult::ErrorDidntReload((
                self,
                format!(
                    "Didn't reload {:?}, since we were unable to get last load time",
                    self.filepath
                ),
            ));
        };
        if self.last_read > last_mtime {
            ReloadResult::NotNeeded(self)
        } else if let Ok(file) = LoadedFile::from_path(&self.filepath) {
            ReloadResult::Reloaded(file)
        } else {
            ReloadResult::ErrorDidntReload((
                self,
                format!(
                    "Didn't reload {:?}, since we were unable to load its contents",
                    self.filepath
                ),
            ))
        }
    }
    pub fn from_path(filepath: &Path) -> anyhow::Result<LoadedFile> {
        let contents = fs::read(filepath)
            .map_err(anyhow::Error::from)
            .context(format!("Unable to read from {:?}", &filepath))?;
        let last_read = SystemTime::now();
        Ok(LoadedFile {
            filepath: filepath.to_path_buf(),
            last_read,
            contents,
        })
    }
}
