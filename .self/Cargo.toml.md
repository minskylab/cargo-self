[package]
name = "cargo-self"
version = "0.1.3"
edition = "2021"
description = "A tool to build and install a package using cargo enhanced with LLM models."
license = "MIT AND Apache-2.0"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
async-openai = "0.14.3"
cargo = "0.74.0"
clap = { version = "4.4.6", features = ["derive"] }
ignore = "0.4.20"
sha2 = "0.10.8"
tokio = { version = "1.33.0", features = ["full"] }
walkdir = "2.4.0"
