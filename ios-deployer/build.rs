use std::env;
use std::fs;
use std::path;

fn main() {
    if !cfg!(feature = "ios") && ! !cfg!(feature = "android") {
        panic!("At least one of `ios` or `android` platform should be enabled by feature!")
    }

    let md_framework_location =
        path::PathBuf::from("/System/Library/PrivateFrameworks/MobileDevice.framework");
    let new_md_framework_location = path::PathBuf::from(
        "/Library/Apple/System/Library/PrivateFrameworks/MobileDevice.framework",
    );
    println!("cargo:rerun-if-changed=build.rs");
    if new_md_framework_location.exists() && new_md_framework_location.is_dir() {
        // check if this new location exists, if it does, copy it instead
        println!(
            "cargo:rustc-link-search=framework={}",
            new_md_framework_location.parent().unwrap().display()
        );
        println!("cargo:rustc-link-lib=framework=MobileDevice");
    } else if md_framework_location.exists() && md_framework_location.is_dir() {
        // check if this new location exists, if it does, copy it instead
        println!(
            "cargo:rustc-link-search=framework={}",
            md_framework_location.parent().unwrap().display()
        );
        println!("cargo:rustc-link-lib=framework=MobileDevice");
    } else {
        panic!("private MobileDevice.framework not found.")
    }
}
