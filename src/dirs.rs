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

    #[doc(hidden)]
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

        let filepath_dirsfile = match filepath {
            Some(ref path) => path.as_ref().into(),
            None => match env::var_os("XDG_CONFIG_HOME") {
                Some(dir) => PathBuf::from(dir).join("user-dirs.dirs"),
                None => home.join(".config/user-dirs.dirs"),
            },
        };

        if !filepath_dirsfile.exists() {
            return Ok(Self { home, vars });
        }

        let entries = DirsFile::new(&filepath_dirsfile).entries()?;
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
        self.get("DESKTOP")
    }

    pub fn documents(&self) -> PathBuf {
        self.get("DOCUMENTS")
    }

    pub fn download(&self) -> PathBuf {
        self.get("DOWNLOAD")
    }

    pub fn music(&self) -> PathBuf {
        self.get("MUSIC")
    }

    pub fn pictures(&self) -> PathBuf {
        self.get("PICTURES")
    }

    pub fn projects(&self) -> PathBuf {
        self.get("PROJECTS")
    }

    pub fn publicshare(&self) -> PathBuf {
        self.get("PUBLICSHARE")
    }

    pub fn templates(&self) -> PathBuf {
        self.get("TEMPLATES")
    }

    pub fn videos(&self) -> PathBuf {
        self.get("VIDEOS")
    }

    pub fn get(&self, dirname: &str) -> PathBuf {
        let name = format!("XDG_{dirname}_DIR");
        match self.vars.get(&name) {
            Some(v) => v.to_string().into(),
            None => self.home.clone(),
        }
    }
}
