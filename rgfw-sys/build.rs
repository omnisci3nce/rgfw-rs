use std::env;
use std::path::PathBuf;

fn main() {
    // Compile the C implementation
    println!("cargo:rerun-if-changed=rgfw_impl.c");
    cc::Build::new().file("rgfw_impl.c").compile("rgfw");
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    let target_os =
        env::var("CARGO_CFG_TARGET_OS").expect("target_os could not be loaded from env");
    if target_os == "linux" {
        println!("cargo:rustc-link-lib=Xrandr");
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=pthread");
    } else if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
