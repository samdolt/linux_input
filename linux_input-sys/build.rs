extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("wrapper.h")
        .generate()
        .expect("Bindgen was unable to generate rust binding to <linux/input.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

}
