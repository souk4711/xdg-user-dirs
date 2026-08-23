#[cfg(test)]
mod lib_test {
    #[test]
    fn test_new() {
        let dirs = xdg_user_dirs::new();
        assert!(dirs.is_ok());
    }
}
