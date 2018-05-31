extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:include=vendor/infinity/release/include");
    println!("cargo:rustc-link-search=vendor/infinity/release");
    println!("cargo:include=helpers/release/include");
    println!("cargo:rustc-link-search=helpers/release");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=ibverbs");
    println!("cargo:rustc-link-lib=infinity");
    println!("cargo:rustc-link-lib=infinityhelpers");

    Command::new("make")
        .args(&["clean"])
        .current_dir("vendor/infinity/")
        .status()
        .expect("Failed to build (clean) infinity");

    Command::new("make")
        .args(&["CC_FLAGS=-O3 -std=c++0x -DINFINITY_ASSERT_ON"])
        .current_dir("vendor/infinity/")
        .status()
        .expect("Failed to build infinity");

    Command::new("make")
        .args(&["clean"])
        .current_dir("helpers/")
        .status()
        .expect("Failed to build (clean) infinity helpers");

    Command::new("make")
        .current_dir("helpers/")
        .status()
        .expect("Failed to build infinity helpers");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // generate the bindings
    bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Ivendor/infinity/release/include/")
        .clang_arg("-Ihelpers/release/include/")
        .clang_arg("-x").clang_arg("c++")
        .clang_arg("-std=c++14")
        .opaque_type("std::.*")
        .whitelist_type("infinity::core::Configuration")
        .whitelist_type("infinity::core::Context")
        .whitelist_type("infinity::queues::QueuePair")
        .whitelist_type("infinity::queues::QueuePairFactory")
        .whitelist_type("infinity::memory::Region")
        .whitelist_type("infinity::memory::RegionToken")
        .whitelist_function("infinityhelpers::.*")
        .opaque_type("infinity::memory::Buffer")
        .enable_cxx_namespaces()
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
