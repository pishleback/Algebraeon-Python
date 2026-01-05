use std::{env, fs, path::PathBuf};

fn main() {
    // Ask Cargo for dependency metadata
    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .expect("failed to read cargo metadata");

    // Find the dependency by name
    let algebraeon_dep = metadata
        .packages
        .iter()
        .find(|p| p.name == "algebraeon")
        .expect("algebraeon not found in dependency graph");

    let algebraeon_version = &algebraeon_dep.version;

    // Where build scripts are allowed to write generated files
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest = out_dir.join("algebraeon_dep_version.rs");

    // Generate Rust source
    fs::write(&dest, format!("{}", algebraeon_version)).expect("failed to write generated file");

    // Re-run build.rs if dependencies change
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=build.rs");
}
