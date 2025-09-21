use std::env;
use std::fs;
use std::process::Command; // For fs::copy

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let current_dir = env::current_dir().unwrap();
    let swift_src_dir = current_dir.join("swift_src");

    let input_manager_o_temp = format!("{}/InputSourceManager.o", current_dir.display()); // Compile to current dir first
    let rust_bridge_o_temp = format!("{}/rust_bridge.o", current_dir.display()); // Compile to current dir first

    // Compile both Swift files together into individual object files in the current directory
    let output = Command::new("swiftc")
        .arg("-c")
        .arg(swift_src_dir.join("InputSourceManager.swift"))
        .arg(swift_src_dir.join("rust_bridge.swift"))
        .arg("-module-name")
        .arg("IsmBridge")
        .output()
        .expect("Failed to compile Swift files");

    if !output.status.success() {
        eprintln!("Swift compilation failed: {:?}", output);
        panic!("Swift compilation failed");
    }

    // Move the compiled object files to OUT_DIR
    fs::copy(
        &input_manager_o_temp,
        format!("{}/InputSourceManager.o", out_dir),
    )
    .expect("Failed to copy InputSourceManager.o to OUT_DIR");
    fs::copy(&rust_bridge_o_temp, format!("{}/rust_bridge.o", out_dir))
        .expect("Failed to copy rust_bridge.o to OUT_DIR");

    // Clean up temporary .o files from current directory
    fs::remove_file(&input_manager_o_temp)
        .expect("Failed to remove temporary InputSourceManager.o");
    fs::remove_file(&rust_bridge_o_temp).expect("Failed to remove temporary rust_bridge.o");

    let input_manager_o = format!("{}/InputSourceManager.o", out_dir);
    let rust_bridge_o = format!("{}/rust_bridge.o", out_dir);

    // Create static library from object files
    let lib_path = format!("{}/libism_bridge.a", out_dir);
    let output = Command::new("ar")
        .arg("rcs")
        .arg(&lib_path)
        .arg(&input_manager_o)
        .arg(&rust_bridge_o)
        .output()
        .expect("Failed to create static library");
    if !output.status.success() {
        eprintln!("Static library creation failed: {:?}", output);
        panic!("Static library creation failed");
    }

    // Link against system frameworks
    println!("cargo:rustc-link-lib=framework=ApplicationServices");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=AppKit");

    // Link against our compiled Swift static library
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ism_bridge");

    // Add search path for Swift runtime libraries
    println!("cargo:rustc-link-search=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
}
