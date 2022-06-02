use std::process::exit;

fn main() {
    let mut failed = false;

    macro_rules! check_struct {
        ($name:ident) => {{
            let pyo3_ffi_size = std::mem::size_of::<pyo3_ffi::$name>();
            let bindgen_size = std::mem::size_of::<bindings::$name>();
            
            let pyo3_ffi_align = std::mem::align_of::<pyo3_ffi::$name>();
            let bindgen_align = std::mem::align_of::<bindings::$name>();
            
            // Check if sizes differ, but ignore zero-sized types (probably "opaque" in pyo3-ffi)
            if pyo3_ffi_size == 0 {
                println!(
                    "warning: ignoring zero-sized pyo3_ffi type {}",
                    stringify!($name),
                );
            } else if pyo3_ffi_size != bindgen_size {
                failed = true;
                println!(
                    "error: size of {} differs between pyo3_ffi ({}) and bindgen ({})",
                    stringify!($name),
                    pyo3_ffi_size,
                    bindgen_size
                );
            } else if pyo3_ffi_align != bindgen_align {
                failed = true;
                println!(
                    "error: alignment of {} differs between pyo3_ffi ({}) and bindgen ({})",
                    stringify!($name),
                    pyo3_ffi_align,
                    bindgen_align
                );
            }
        }};
    }

    pyo3_ffi_check_macro::for_all_structs!(check_struct);

    if failed {
        exit(1);
    } else {
        exit(0);
    }
}

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, improper_ctypes)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
