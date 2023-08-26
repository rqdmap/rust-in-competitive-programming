use std::process::Command;
use std::path::Path;
use rustsourcebundler::Bundler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bundler: Bundler = Bundler::new(
        Path::new("src/bin/main.rs"),
        Path::new("./bundle.rs"),
    );
    bundler.crate_name("rust-in-competitive-programming");
    bundler.run();

    Command::new("rustfmt")
        .arg("--edition=2018")
        .arg("./bundle.rs")
        .output()
        .expect("Failed to execute rustfmt");
    Ok(())
}
