use rust_bundler::Bundler;
fn main() {
    let mut bundler = Bundler::new(
        "rust_in_competitive_programming",
        "src/main.rs",
        "bundle/main-bundle.rs",
        true,
    );

    bundler.set_banner("src/misc/banner.rs");

    bundler.run();
}

