use rust_bundler::Bundler;
fn main() {
    let mut bundler = Bundler::new(
        "rust_in_competitive_programming",
        "src/bin/main.rs",
        "bundle/main-bundle.rs"
    );

    bundler.run();
}

