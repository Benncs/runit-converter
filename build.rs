//
fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-env=MYAPP_DB_PATH=/usr/share/myapp/units.db");
    } else {
        println!(
            "cargo:rustc-env=DB_PATH={}/data/sqlite.db",
            env!("CARGO_MANIFEST_DIR")
        );
    }
}
