use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=SPIRV_TOOLS_DIR");
    let dir = env::var("SPIRV_TOOLS_DIR").expect("SPIRV_TOOLS_DIR must be set");
    println!("cargo:rustc-link-search=native={dir}/lib");
    println!("cargo:rustc-link-lib=SPIRV-Tools-opt");
    println!("cargo:rustc-link-lib=SPIRV-Tools");
    println!("cargo:rustc-link-lib=stdc++");
    let bindings = bindgen::Builder::default()
        // This is recommended I think?
        .clang_arg("std=c++14")
        // The input header we would like to generate
        // bindings for.
        .headers([format!("{dir}/include/spirv-tools/libspirv.h")])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Error generating bindings for spirv-tools");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings to file");
}
