use std::process::Command;
use std::path::Path;
use rustsourcebundler::Bundler;

macro_rules! _p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(entries) = std::fs::read_dir("./src/bin") {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.path();

                let a = entry.file_name();
                let a = "./bundle/".to_string() + &a.to_string_lossy().split(".").collect::<Vec<&str>>()[0].to_string() + "-bundle.rs";

                let mut bundler: Bundler = Bundler::new(
                    Path::new(&file_name),
                    Path::new(&a),
                );

                bundler.crate_name("rust-in-competitive-programming");
                bundler.run();
                Command::new("rustfmt")
                    .arg("--edition=2018")
                    .arg(a)
                    .output()
                    .expect("Failed to execute rustfmt");
            } }
    }
    Ok(())
}
