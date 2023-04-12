extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    //cargo add --build bindgen
    // println!("cargo:rerun-if-changed=src/hello.h");
    // println!("cargo:rerun-if-changed=src/hello.c");

    let mut builder: cc::Build = cc::Build::new();
    builder
        .file("./src/hello.c")
        .shared_flag(false)
        .compile("hello");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/hello.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = env::current_dir().unwrap();
    bindings
        .write_to_file(out_path.join("src\\bindings.rs"))
        .expect("Couldn't write bindings!");
}
