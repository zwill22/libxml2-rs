use cmake::Config;
use std::env;
use std::path::{Path, PathBuf};

fn fetch_lib_path() -> Option<String> {
    if let Ok(lib_path) = env::var("LIBXML2_INCLUDE_DIR") {
        if Path::new(&lib_path).exists() {
            return Some(lib_path.to_string());
        }
    }

    let lib_path = "/usr/include/libxml2";
    if Path::new(lib_path).exists() {
        return Some(lib_path.to_string());
    }

    None
}

fn generate_config() -> Config {
    let mut config = Config::new("libxml2_interface");

    if let Ok(cmake_toolchain_file) = env::var("CMAKE_TOOLCHAIN_FILE") {
        config.define("CMAKE_TOOLCHAIN_FILE", cmake_toolchain_file);
    }

    if let Ok(out_dir) = env::var("OUT_DIR") {
        config.out_dir(out_dir);
    }

    config
}

fn main() {
    let path = generate_config().build();

    println!("cargo:rustc-link-search=native={:?}", path);
    println!("cargo:rustc-link-lib=xml2");

    let callbacks = bindgen::CargoCallbacks::new();

    let builder = bindgen::builder()
        .header("libxml2_interface/wrapper.h")
        .parse_callbacks(Box::new(callbacks));

    let bindings = match fetch_lib_path() {
        Some(lib_path) => builder.clang_arg(format!("-I{}", lib_path)),
        None => builder,
    }
    .generate()
    .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
