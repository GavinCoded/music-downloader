use std::process::Command;

fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    let status = Command::new("glib-compile-resources")
        .arg("--sourcedir=data")
        .arg(&format!("--target={out}/resources.gresource"))
        .arg("data/resources.gresource.xml")
        .status()
        .expect("glib-compile-resources not found");
    assert!(status.success(), "glib-compile-resources failed");
    println!("cargo:rerun-if-changed=data/resources.gresource.xml");
    println!("cargo:rerun-if-changed=data/icons/hicolor/symbolic/apps/spotify-symbolic.svg");
}
