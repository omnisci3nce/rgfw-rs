use std::env;
use std::path::PathBuf;

fn main() {
    // Compile the C implementation
    println!("cargo:rerun-if-changed=rgfw_impl.c");
    cc::Build::new().file("rgfw_impl.c").compile("rgfw");
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    // TODO: Figure out how to make cross-platform. Can we use the Makefile in some way?
    println!("cargo:rustc-link-lib=Xrandr");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");

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
