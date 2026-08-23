use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

use crate::{DirsFile, error::*};

#[derive(Debug)]
pub struct Dirs {
    home: PathBuf,
    vars: HashMap<String, String>,
}

impl Dirs {
    pub fn new() -> Result<Self> {
        Self::from_file_imp::<PathBuf>(None)
    }

    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Result<Self> {
        Self::from_file_imp(Some(filepath))
    }

    fn from_file_imp<P: AsRef<Path>>(filepath: Option<P>) -> Result<Self> {
        let home = match env::home_dir() {
            Some(dir) => dir,
            None => return Err(Error::NoHome),
        };

        let h = home.to_string_lossy().to_string();
        let mut vars = HashMap::from([
            ("XDG_DESKTOP_DIR".to_string(), format!("{h}/Desktop")),
            ("XDG_DOCUMENTS_DIR".to_string(), format!("{h}/Documents")),
            ("XDG_DOWNLOAD_DIR".to_string(), format!("{h}/Downloads")),
            ("XDG_MUSIC_DIR".to_string(), format!("{h}/Music")),
            ("XDG_PICTURES_DIR".to_string(), format!("{h}/Pictures")),
            ("XDG_PROJECTS_DIR".to_string(), format!("{h}/Projects")),
            ("XDG_PUBLICSHARE_DIR".to_string(), format!("{h}/Public")),
            ("XDG_TEMPLATES_DIR".to_string(), format!("{h}/Templates")),
            ("XDG_VIDEOS_DIR".to_string(), format!("{h}/Videos")),
        ]);

        let filepath = match filepath {
            Some(filepath) => filepath.as_ref().into(),
            None => match env::var_os("XDG_CONFIG_HOME") {
                Some(dir) => PathBuf::from(dir).join("user-dirs.dirs"),
                None => home.join(".config/user-dirs.dirs"),
            },
        };

        let entries = DirsFile::new(&filepath).entries()?;
        for entry in &entries {
            let value = match entry.value.strip_prefix("$HOME/") {
                Some(v) => format!("{h}/{v}"),
                None => entry.value.clone(),
            };
            vars.insert(entry.name.clone(), value);
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
