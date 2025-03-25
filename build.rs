use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("lokbridge_impl.c")
        .include(".")
        .compile("lokbridge");

    println!("cargo:rustc-link-lib=static=lokbridge");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-search=./headers");

    let bindings = bindgen::Builder::default()
        .header("lokbridge.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src/generated");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
