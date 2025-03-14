extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod parse;
use parse::Parsed;

mod flog;
#[proc_macro]
pub fn flog(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Parsed);
    flog::flog_impl(parsed)
}

mod attach;
#[proc_macro]
pub fn attach(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Parsed);
    attach::attach_impl(parsed)
}
mod define;
#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
    define::define_impl(input)
}
