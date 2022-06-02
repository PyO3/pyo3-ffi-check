use std::env;
use std::path::PathBuf;

fn main() {
    let config = pyo3_build_config::get();
    let python_include_dir = config.run_python_script("import sysconfig; print(sysconfig.get_config_var('INCLUDEPY'), end='');").expect("failed to get lib dir");

    println!("cargo:rerun-if-changed=wrapper.h");
    dbg!(format!("-I{python_include_dir}"));

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{python_include_dir}"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
