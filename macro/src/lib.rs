use proc_macro2::{TokenStream, Span, Ident, TokenTree};
use quote::quote;

/// Macro which expands to multiple macro calls, one per pyo3-ffi struct.
#[proc_macro]
pub fn for_all_structs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let mut input = input.into_iter();

    let macro_name = match input.next() {
        Some(TokenTree::Ident(i)) => i,
        _ => return quote!(
            compile_error!("generated_test!() takes only a single ident as input")
        ).into()
    };

    if !input.next().is_none() {
        return quote!(
            compile_error!("generated_test!() takes only a single ident as input")
        ).into();
    }

    let mut output = TokenStream::new();

    for entry in glob::glob("target/doc/pyo3_ffi/struct.*.html").expect("Failed to read glob pattern") {
        let entry = entry.unwrap().file_name().unwrap().to_string_lossy().into_owned();
        let struct_name = entry.strip_prefix("struct.").unwrap().strip_suffix(".html").unwrap();
        let struct_ident = Ident::new(struct_name, Span::call_site());
        output.extend(quote!(#macro_name!(#struct_ident);));
    }

    if output.is_empty() {
        quote!(
            compile_error!(
                "No files found in `target/doc/pyo3_ffi/struct.*.html`, try running \
                 `cargo doc -p pyo3_ffi` first."
            )
        )
    } else {
        output
    }.into()
}
