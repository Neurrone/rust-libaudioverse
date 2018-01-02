#[cfg(feature = "bindgen")]
extern crate bindgen;

use std::path::PathBuf;
use std::{ fs, env };

fn main() {
    // Tell cargo to tell rustc to link the system libaudioverse shared library.
    println!("cargo:rustc-link-lib=libaudioverse");
    
    #[cfg(feature = "bindgen")] {
        generate_bindings();
    }
    
    #[cfg(not(feature = "bindgen"))] {
        copy_pregenerated_bindings();
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");
    
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "bindgen"))]
fn copy_pregenerated_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(crate_path.join("pregenerated_bindings.rs"), out_path.join("bindings.rs"))
        .expect("Couldn't find pregenerated bindings!");
}