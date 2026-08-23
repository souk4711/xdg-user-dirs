# xdg-user-dirs

A library that looks up the current path for one of the XDG user directories.

## Usage

```rust
let dirs = xdg_user_dirs::new()?;

dirs.desktop()      # $HOME/Desktop
dirs.documents()    # $HOME/Documents
dirs.downloads()    # $HOME/Downloads
dirs.music()        # $HOME/Music
dirs.pictures()     # $HOME/Pictures
dirs.projects()     # $HOME/Projects
dirs.publicshare()  # $HOME/Public
dirs.templates()    # $HOME/Templates
dirs.videos()       # $HOME/Videos

dirs.get("CUSTOM")  # $HOME/Custom if set
dirs.get("CUSTOM")  # $HOME if unset
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
