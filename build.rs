use cmake::Config;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let path = Config::new("libxml2_interface")
        .out_dir(env::var("OUT_DIR").unwrap())
        .build();

    println!("cargo:rustc-link-search=native={:?}", path);
    println!("cargo:rustc-link-lib=xml2");

    let builder = bindgen::builder()
        .header("libxml2_interface/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let bindings: bindgen::Bindings;
    let lib_path = "/usr/include/libxml2";
    if Path::new(lib_path).exists() {
        bindings = builder
            .clang_arg(format!("-I{}", lib_path))
            .generate()
            .expect("Unable to generate bindings");
    } else {
        bindings = builder.generate().expect("Unable to generate bindings");
    }

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
