//
fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-env=DB_PATH={}", env!("DB_PATH"));
    } else {
        println!(
            "cargo:rustc-env=DB_PATH={}/data/sqlite.db",
            env!("CARGO_MANIFEST_DIR")
        );
    }
}
