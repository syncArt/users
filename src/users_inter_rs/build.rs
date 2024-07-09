use ic_cdk_bindgen::{Builder, Config};
use std::path::PathBuf;

fn main() {
    // A workaround to force always rerun build.rs
    println!("cargo:rerun-if-changed=NULL");
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("Cannot find manifest dir"));
    let users_rs = Config::new("users_rs");
    let mut builder = Builder::new();
    builder.add(users_rs);
    builder.build(Some(manifest_dir.join("declarations")));
}