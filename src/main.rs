macro_rules! check_struct {
    ($name:ident) => {
        println!("{} - {} - {}", stringify!($name), std::mem::size_of::<pyo3_ffi::$name>(), std::mem::size_of::<bindings::$name>());
    };
}

fn main() {
    pyo3_binary_check_macro::for_all_structs!(check_struct);
}

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
