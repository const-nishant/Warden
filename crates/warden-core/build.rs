use std::{env, path::PathBuf};

fn main() {
    let cargo_manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_dir = cargo_manifest
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("proto");

    let proto_file = proto_dir.join("warden").join("v1").join("chat_frame.proto");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_protos(&[&proto_file], &[&proto_dir])
        .unwrap();

    println!("cargo:rerun-if-changed={}", proto_file.display());
}
