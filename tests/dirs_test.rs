#[cfg(test)]
mod dirs_test {
    use std::env;

    use xdg_user_dirs::Dirs;

    #[test]
    fn test_new() {
        let dirs = Dirs::new();
        assert!(dirs.is_ok());
    }

    #[test]
    fn test_from_file() {
        let dirs = Dirs::from_file(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/user-dirs.dirs.defaults"
        ));
        assert!(dirs.is_ok());

        let home = env::home_dir().unwrap().to_string_lossy().to_string();
        let dirs = dirs.unwrap();
        assert_eq!(dirs.desktop(), format!("{home}/Desktop"));
        assert_eq!(dirs.documents(), format!("{home}/Documents"));
        assert_eq!(dirs.downloads(), format!("{home}/Downloads"));
        assert_eq!(dirs.music(), format!("{home}/Music"));
        assert_eq!(dirs.pictures(), format!("{home}/Pictures"));
        assert_eq!(dirs.projects(), format!("{home}/Projects"));
        assert_eq!(dirs.publicshare(), format!("{home}/Public"));
        assert_eq!(dirs.templates(), format!("{home}/Templates"));
        assert_eq!(dirs.videos(), format!("{home}/Videos"));
    }

    #[test]
    fn test_from_file_en() {
        let dirs = Dirs::from_file(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/user-dirs.dirs.locale.en_US"
        ));
        assert!(dirs.is_ok());

        let home = env::home_dir().unwrap().to_string_lossy().to_string();
        let dirs = dirs.unwrap();
        assert_eq!(dirs.desktop(), format!("{home}/Desktop"));
        assert_eq!(dirs.documents(), format!("{home}/Documents"));
        assert_eq!(dirs.downloads(), format!("{home}/Downloads"));
        assert_eq!(dirs.music(), format!("{home}/Music"));
        assert_eq!(dirs.pictures(), format!("{home}/Pictures"));
        assert_eq!(dirs.projects(), format!("{home}/Projects"));
        assert_eq!(dirs.publicshare(), format!("{home}/Public"));
        assert_eq!(dirs.templates(), format!("{home}/Templates"));
        assert_eq!(dirs.videos(), format!("{home}/Videos"));
    }

    #[test]
    fn test_from_file_zh() {
        let dirs = Dirs::from_file(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/user-dirs.dirs.locale.zh_HK"
        ));
        assert!(dirs.is_ok());

        let home = env::home_dir().unwrap().to_string_lossy().to_string();
        let dirs = dirs.unwrap();
        assert_eq!(dirs.desktop(), format!("{home}/桌面"));
        assert_eq!(dirs.documents(), format!("{home}/文件"));
        assert_eq!(dirs.downloads(), format!("{home}/下載"));
        assert_eq!(dirs.music(), format!("{home}/音樂"));
        assert_eq!(dirs.pictures(), format!("{home}/圖片"));
        assert_eq!(dirs.projects(), format!("{home}/專案"));
        assert_eq!(dirs.publicshare(), format!("{home}/公共"));
        assert_eq!(dirs.templates(), format!("{home}/模板"));
        assert_eq!(dirs.videos(), format!("{home}/影片"));
    }

    #[test]
    fn test_from_file_filenotexist() {
        let dirs = Dirs::from_file(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/user-dirs.dirs.filenotexist"
        ));
        assert!(!dirs.is_ok());
    }
}
