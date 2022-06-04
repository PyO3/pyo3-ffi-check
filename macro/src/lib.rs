use std::{env, fs, path::PathBuf};

use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::quote;

/// Macro which expands to multiple macro calls, one per pyo3-ffi struct.
#[proc_macro]
pub fn for_all_structs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let mut input = input.into_iter();

    let macro_name = match input.next() {
        Some(TokenTree::Ident(i)) => i,
        _ => {
            return quote!(compile_error!(
                "generated_test!() takes only a single ident as input"
            ))
            .into()
        }
    };

    if !input.next().is_none() {
        return quote!(compile_error!(
            "generated_test!() takes only a single ident as input"
        ))
        .into();
    }

    let doc_dir = get_doc_dir();
    let structs_glob = format!("{}/doc/pyo3_ffi/struct.*.html", doc_dir.display());

    let mut output = TokenStream::new();

    for entry in glob::glob(&structs_glob).expect("Failed to read glob pattern") {
        let entry = entry
            .unwrap()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let struct_name = entry
            .strip_prefix("struct.")
            .unwrap()
            .strip_suffix(".html")
            .unwrap();
        let struct_ident = Ident::new(struct_name, Span::call_site());
        output.extend(quote!(#macro_name!(#struct_ident);));
    }

    if output.is_empty() {
        quote!(compile_error!(concat!(
            "No files found at `",
            #structs_glob,
            "`, try running `cargo doc -p pyo3-ffi` first."
        )))
    } else {
        output
    }
    .into()
}

fn get_doc_dir() -> PathBuf {
    let path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    path.parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

/// Macro which expands to multiple macro calls, one per field in a pyo3-ffi
/// struct.
#[proc_macro]
pub fn for_all_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let mut input = input.into_iter();

    let struct_name = match input.next() {
        Some(TokenTree::Ident(i)) => i,
        _ => {
            return quote!(compile_error!(
                "generated_test!() takes exactly two ident as input"
            ))
            .into()
        }
    };

    match input.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => (),
        _ => {
            return quote!(compile_error!(
                "generated_test!() takes exactly two ident as input"
            ))
            .into()
        }
    };

    let macro_name = match input.next() {
        Some(TokenTree::Ident(i)) => i,
        _ => {
            return quote!(compile_error!(
                "generated_test!() takes exactly two ident as input"
            ))
            .into()
        }
    };

    if !input.next().is_none() {
        return quote!(compile_error!(
            "generated_test!() takes exactly two ident as input"
        ))
        .into();
    }

    let doc_dir = get_doc_dir();
    let struct_file = fs::read_to_string(format!(
        "{}/doc/pyo3_ffi/struct.{}.html",
        doc_dir.display(),
        struct_name
    ))
    .unwrap();

    let html = scraper::Html::parse_document(&struct_file);
    let selector = scraper::Selector::parse("span.structfield").unwrap();

    let mut output = TokenStream::new();

    for el in html.select(&selector) {
        let id = el
            .value()
            .id()
            .unwrap()
            .strip_prefix("structfield.")
            .unwrap();

        let field_ident = Ident::new(id, Span::call_site());

        output.extend(quote!(#macro_name!(#struct_name, #field_ident);));
    }

    output.into()
}
