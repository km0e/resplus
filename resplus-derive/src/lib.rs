extern crate proc_macro;

use core::panic;
use std::borrow::Cow;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ExprCall, Token, parse_macro_input, parse_str};

use proc_macro2::TokenStream as TokenStream2;

#[proc_macro]
pub fn flog(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ParsedMap);
    quote!(#input).into()
}

struct PrintArg {
    idx: Expr,
    arg: Expr,
}

impl PrintArg {
    fn new(idx: Expr, arg: Expr) -> Self {
        PrintArg { idx, arg }
    }
}

impl ToTokens for PrintArg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let idx = &self.idx;
        let arg = &self.arg;
        tokens.extend(quote! {
            let #idx = #arg;
        });
    }
}

struct ParsedMap {
    fmt: String,
    func: ExprCall,
    pargs: Vec<PrintArg>,
}

impl ToTokens for ParsedMap {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pargs = &self.pargs;
        let func = &self.func;
        let fmt = &self.fmt;
        tokens.extend(quote! {
            {
                use resplus::ResultChain;
                #(#pargs)*
                #func.about_else(|| format!(#fmt))?
            }
        });
    }
}

struct PrintIndex {
    idx: usize,
}

impl Parse for PrintIndex {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![,]>()?;
        let idx = input
            .parse::<syn::LitInt>()?
            .base10_parse()
            .expect("parse idx");
        Ok(PrintIndex { idx })
    }
}

impl Parse for ParsedMap {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            panic!("There should be a function call");
        }
        let mut func = input.parse::<ExprCall>()?;
        let mut fargs = Vec::new();
        let mut pargs = Vec::new();

        let mut skip_idx = 0;
        while let Ok(idx) = input.parse::<PrintIndex>() {
            while skip_idx != idx.idx {
                fargs.push(Cow::Borrowed("_"));
                skip_idx += 1;
            }
            let tmp_id_str = format!("__{}", idx.idx);
            let tmp_id = parse_str::<Expr>(&tmp_id_str).unwrap();
            pargs.push(PrintArg::new(
                tmp_id.clone(),
                std::mem::replace(&mut func.args[idx.idx], tmp_id),
            ));
            fargs.push(format!("{{{tmp_id_str}}}").into());
            skip_idx += 1;
        }
        while skip_idx < func.args.len() {
            fargs.push("_".into());
            skip_idx += 1;
        }

        let fmt = format!("{}({})", func.func.to_token_stream(), fargs.join(", "));
        Ok(ParsedMap { func, fmt, pargs })
    }
}
