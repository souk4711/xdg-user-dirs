# xdg-user-dirs

A library that looks up the current path for one of the XDG user directories.

## Usage

```rust
fn main() {
  let dirs = xdg_user_dirs::new().unwrap();

  println!("    XDG_DESKTOP_DIR: {:?}", dirs.desktop());      // /home/johndoe/Desktop
  println!("  XDG_DOCUMENTS_DIR: {:?}", dirs.documents());    // /home/johndoe/Documents
  println!("   XDG_DOWNLOAD_DIR: {:?}", dirs.download());     // /home/johndoe/Downloads
  println!("      XDG_MUSIC_DIR: {:?}", dirs.music());        // /home/johndoe/Music
  println!("   XDG_PICTURES_DIR: {:?}", dirs.pictures());     // /home/johndoe/Pictures
  println!("   XDG_PROJECTS_DIR: {:?}", dirs.projects());     // /home/johndoe/Projects
  println!("XDG_PUBLICSHARE_DIR: {:?}", dirs.publicshare());  // /home/johndoe/Public
  println!("  XDG_TEMPLATES_DIR: {:?}", dirs.templates());    // /home/johndoe/Templates
  println!("     XDG_VIDEOS_DIR: {:?}", dirs.videos());       // /home/johndoe/Videos

  // /home/johndoe/Custom if set
  // /home/johndoe if unset
  println!("     XDG_CUSTOM_DIR: {:?}", dirs.get("CUSTOM"));
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
