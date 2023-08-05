## Learning Rust
- `/src/section_<number>_<order>_name.rs`, the separate modules relevant to the study topics, can be imported and run in `main.rs`

## adding new package

goto `Cargo.toml`

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# here below you add the packages:
rand = "0.8.5"
```