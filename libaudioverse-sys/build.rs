#[cfg(feature = "bindgen")]
extern crate bindgen;

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let libaudioverse_libs_dir = env::var("LIBAUDIOVERSE_LIBS").unwrap_or_else(|_| {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("lib")
            .to_string_lossy()
            .to_string()
    });
    
    println!("cargo:rustc-link-lib=libaudioverse");
    println!("cargo:rustc-link-search={}", libaudioverse_libs_dir);
    
    #[cfg(feature = "bindgen")] {
        generate_bindings();
    }
    
    #[cfg(not(feature = "bindgen"))] {
        copy_pregenerated_bindings();
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    use std::path;
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .generate_comments(true)
        .whitelist_recursively(false)
        .rust_target(bindgen::RustTarget::Nightly)
        .enable_cxx_namespaces()
        // Re-structure the modules a bit and hide the "root" module
        .raw_line("#[doc(hidden)]")
        .derive_debug(true)
        .derive_hash(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .rustfmt_bindings(true)
        .generate()
        .expect("unable to generate libaudioverse bindings");
    
    let out_path = path::PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR env var not set"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings file");

    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(
        out_path.join("bindings.rs"),
        crate_path.join("pregenerated_bindings.rs"),
    ).expect("Couldn't find generated bindings!");
}

#[cfg(not(feature = "bindgen"))]
fn copy_pregenerated_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(
        crate_path.join("pregenerated_bindings.rs"),
        out_path.join("bindings.rs"),
    ).expect("Couldn't find pregenerated bindings!");
}
