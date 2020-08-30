extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn};

/// Instrument selected function
///
/// Example:
/// #[optick_attr::profile]
/// fn calc() {
///	    // Do some stuff
///}
#[proc_macro_attribute]
#[cfg(all(feature = "enable"))]
pub fn profile(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);

    let body = &function.block;
    let new_body: syn::Block = parse_quote! {
        {
            optick::event!();
            #body
        }
    };

    function.block = Box::new(new_body);

    (quote! {
        #function
    })
    .into()
}

/// Generate profiling capture for the selected function
/// Saves capture to {working_dir}/capture_name(date-time).opt
/// Note: You could use full path for the name (e.g. "D:/captures/game")
///
/// Example:
/// #[optick_attr::capture("capture_name")]
/// pub fn main() {
///     calc();
/// }
#[proc_macro_attribute]
#[cfg(all(feature = "enable"))]
pub fn capture(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path_arg = attr.to_string();
    let path_length = path_arg.len();
    let path = if path_length < 2 {
        "optick_capture"
    } else {
        &path_arg[1..path_length - 1]
    };
    let mut function = parse_macro_input!(item as ItemFn);
    let body = &function.block;
    let new_body: syn::Block = parse_quote! {
        {
            optick::start_capture();
            let result = #body;
            optick::stop_capture(#path);
            result
        }
    };

    function.block = Box::new(new_body);

    (quote! {
        #function
    })
    .into()
}
