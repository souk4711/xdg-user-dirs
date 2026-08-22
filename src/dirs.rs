use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

use crate::{DirsFile, error::*};

pub struct Dirs {
    home: PathBuf,
    vars: HashMap<String, String>,
}

impl Dirs {
    pub fn from_file<P: AsRef<Path>>(filepath: Option<P>) -> Result<Self> {
        let home = match env::home_dir() {
            Some(dir) => dir,
            None => return Err(Error::NoHome),
        };

        let filepath = match filepath {
            Some(filepath) => filepath.as_ref().into(),
            None => match env::var_os("XDG_CONFIG_HOME") {
                Some(dir) => PathBuf::from(dir).join("user-dirs.dirs"),
                None => home.join(".config/user-dirs.dirs"),
            },
        };

        let mut vars = HashMap::new();
        let h = home.to_string_lossy().to_string();

        let entries = DirsFile::new(&filepath).entries()?;
        for entry in entries {
            let value = match entry.value.strip_prefix("$HOME/") {
                Some(v) => format!("{h}/{v}"),
                None => entry.value,
            };
            vars.insert(entry.name, value);
        }

        Ok(Self { home, vars })
    }

    pub fn desktop(&self) -> PathBuf {
        self.dir("DESKTOP")
    }

    pub fn documents(&self) -> PathBuf {
        self.dir("DOCUMENTS")
    }

    pub fn downloads(&self) -> PathBuf {
        self.dir("DOWNLOAD")
    }

    pub fn music(&self) -> PathBuf {
        self.dir("MUSIC")
    }

    pub fn pictures(&self) -> PathBuf {
        self.dir("PICTURES")
    }

    pub fn projects(&self) -> PathBuf {
        self.dir("PROJECTS")
    }

    pub fn publicshare(&self) -> PathBuf {
        self.dir("PUBLICSHARE")
    }

    pub fn templates(&self) -> PathBuf {
        self.dir("TEMPLATES")
    }

    pub fn videos(&self) -> PathBuf {
        self.dir("VIDEOS")
    }

    pub fn dir(&self, dirname: &str) -> PathBuf {
        let name = format!("XDG_{dirname}_DIR");
        match self.vars.get(&name) {
            Some(v) => v.to_string().into(),
            None => self.home.clone(),
        }
    }
}
