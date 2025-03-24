use std::path::PathBuf;

fn main() {
    // Compile the C implementation
    cc::Build::new()
        .file("lokbridge_impl.c") // This will include lokbridge.h
        .include(".") // Include current directory
        .compile("lokbridge"); // Creates liblokbridge.a

    // Tell cargo to link with the compiled library
    println!("cargo:rustc-link-lib=static=lokbridge");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-search=./headers");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("lokbridge.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("output");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
