use core::panic;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=VULKAN_SDK");
    println!("cargo:rerun-if-env-changed=SPIRV_TOOLS");
    println!("cargo:rerun-if-env-changed=SPIRV_TOOLS_LIBS_DIR");
    println!("cargo:rerun-if-env-changed=SPIRV_TOOLS_HEADERS_DIR");

    let headers_dir = if let Ok(dir) = env::var("SPIRV_TOOLS_HEADERS_DIR") {
        dir
    } else if let Ok(dir) = env::var("SPIRV_TOOLS_DIR") {
        #[cfg(target_os = "windows")]
        {
            format!("{dir}/Include")
        }
        #[cfg(not(target_os = "windows"))]
        {
            format!("{dir}/include")
        }
    } else if let Ok(dir) = env::var("VULKAN_SDK") {
        #[cfg(target_os = "windows")]
        {
            format!("{dir}/Include/spirv-tools")
        }
        #[cfg(not(target_os = "windows"))]
        {
            format!("{dir}/include/spirv-tools")
        }
    } else {
        panic!(
            "The environment variable SPIRV_TOOLS_HEADERS_DIR, SPIRV_TOOLS_DIR, or VULKAN_SDK must be set"
        );
    };
    println!("{headers_dir}");
    let libs_dir = if let Ok(dir) = env::var("SPIRV_TOOLS_LIBS_DIR") {
        dir
    } else if let Ok(dir) = env::var("SPIRV_TOOLS_DIR") {
        #[cfg(target_os = "windows")]
        {
            format!("{dir}/Lib")
        }
        #[cfg(not(target_os = "windows"))]
        {
            format!("{dir}/lib")
        }
    } else if let Ok(dir) = env::var("VULKAN_SDK") {
        #[cfg(target_os = "windows")]
        {
            format!("{dir}/Lib")
        }
        #[cfg(not(target_os = "windows"))]
        {
            format!("{dir}/lib")
        }
    } else {
        panic!(
            "The environment variable SPIRV_TOOLS_LIBS_DIR, SPIRV_TOOLS_DIR, or VULKAN_SDK must be set"
        );
    };
    if !libs_dir.is_empty() {
        println!("cargo:rustc-link-search=native={libs_dir}");
    }
    println!("cargo:rustc-link-lib=SPIRV-Tools-opt");
    println!("cargo:rustc-link-lib=SPIRV-Tools");
    println!("cargo:rustc-link-lib=stdc++");
    let bindings = bindgen::Builder::default()
        // This is recommended I think?
        .clang_arg("std=c++14")
        // The input header we would like to generate
        // bindings for.
        .headers([format!("{headers_dir}/libspirv.h")])
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
