fn main() {
    let dirs = xdg_user_dirs::new().unwrap();

    println!("    XDG_DESKTOP_DIR: {:?}", dirs.desktop());      // home/johndoe/Desktop
    println!("  XDG_DOCUMENTS_DIR: {:?}", dirs.documents());    // home/johndoe/Documents
    println!("   XDG_DOWNLOAD_DIR: {:?}", dirs.download());     // home/johndoe/Downloads
    println!("      XDG_MUSIC_DIR: {:?}", dirs.music());        // home/johndoe/Music
    println!("   XDG_PICTURES_DIR: {:?}", dirs.pictures());     // home/johndoe/Pictures
    println!("   XDG_PROJECTS_DIR: {:?}", dirs.projects());     // home/johndoe/Projects
    println!("XDG_PUBLICSHARE_DIR: {:?}", dirs.publicshare());  // home/johndoe/Public
    println!("  XDG_TEMPLATES_DIR: {:?}", dirs.templates());    // home/johndoe/Templates
    println!("     XDG_VIDEOS_DIR: {:?}", dirs.videos());       // /home/johndoe/Videos

    // home/johndoe/Custom if set
    // home/johndoe if unset
    println!("     XDG_CUSTOM_DIR: {:?}", dirs.get("CUSTOM"));
}

#[test]
fn test_main() {
    main();
}
