extern crate proc_macro;

use core::panic;
use std::borrow::Cow;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, ExprCall, ExprRange, LitInt, Token, parse_macro_input, parse_str};

use proc_macro2::TokenStream as TokenStream2;

#[proc_macro]
pub fn flog(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Parsed);
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

struct Parsed {
    fmt: String,
    func: ExprCall,
    pargs: Vec<PrintArg>,
}

impl ToTokens for Parsed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pargs = &self.pargs;
        let func = &self.func;
        let fmt = &self.fmt;
        tokens.extend(quote! {
            {
                use resplus::ResultChain;
                #[cfg(feature = "async")]
                use resplus::FutResultChain;
                #(#pargs)*
                let __res = #func.about_else(|| format!(#fmt));
                __res
            }
        });
    }
}

impl Parse for Parsed {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            panic!("There should be a function call");
        }
        let mut func = input.parse::<ExprCall>()?;
        let mut fargs = Vec::new();
        let mut pargs = Vec::new();

        let mut skip_idx = 0;
        while input.parse::<Token![,]>().is_ok() {
            if let Ok(range) = input.fork().parse::<ExprRange>() {
                input.parse::<ExprRange>()?;
                let start = range
                    .start
                    .map(|s| {
                        if let syn::Expr::Lit(lit) = s.as_ref() {
                            if let syn::Lit::Int(lit) = &lit.lit {
                                return lit.base10_parse::<usize>();
                            }
                        }
                        Err(syn::Error::new(s.span(), "expected integer"))
                    })
                    .transpose()?
                    .unwrap_or(0);
                while skip_idx < start {
                    fargs.push(Cow::Borrowed("_"));
                    skip_idx += 1;
                }
                let end = range
                    .end
                    .map(|s| {
                        if let syn::Expr::Lit(lit) = s.as_ref() {
                            if let syn::Lit::Int(lit) = &lit.lit {
                                return lit.base10_parse::<usize>();
                            }
                        }
                        Err(syn::Error::new(s.span(), "expected integer"))
                    })
                    .transpose()?
                    .unwrap_or(func.args.len());
                for i in start..end {
                    let tmp_id_str = format!("__{}", i);
                    let tmp_id = parse_str::<Expr>(&tmp_id_str).unwrap();
                    pargs.push(PrintArg::new(
                        tmp_id.clone(),
                        std::mem::replace(&mut func.args[i], tmp_id),
                    ));
                    fargs.push(format!("{{{tmp_id_str}}}").into());
                    skip_idx += 1;
                }
            } else if let Ok(idx) = input.parse::<LitInt>() {
                let idx = idx.base10_parse::<usize>().unwrap();
                while skip_idx != idx {
                    fargs.push(Cow::Borrowed("_"));
                    skip_idx += 1;
                }
                let tmp_id_str = format!("__{}", idx);
                let tmp_id = parse_str::<Expr>(&tmp_id_str).unwrap();
                pargs.push(PrintArg::new(
                    tmp_id.clone(),
                    std::mem::replace(&mut func.args[idx], tmp_id),
                ));
                fargs.push(format!("{{{tmp_id_str}}}").into());
                skip_idx += 1;
            } else {
                Err(syn::Error::new(input.span(), "expected integer or range"))?;
                break;
            }
        }
        // while let Ok(idx) = input.parse::<PrintIndex>() {
        //     while skip_idx != idx.idx {
        //         fargs.push(Cow::Borrowed("_"));
        //         skip_idx += 1;
        //     }
        //     let tmp_id_str = format!("__{}", idx.idx);
        //     let tmp_id = parse_str::<Expr>(&tmp_id_str).unwrap();
        //     pargs.push(PrintArg::new(
        //         tmp_id.clone(),
        //         std::mem::replace(&mut func.args[idx.idx], tmp_id),
        //     ));
        //     fargs.push(format!("{{{tmp_id_str}}}").into());
        //     skip_idx += 1;
        // }
        while skip_idx < func.args.len() {
            fargs.push("_".into());
            skip_idx += 1;
        }

        let fmt = format!("{}({})", func.func.to_token_stream(), fargs.join(", "));
        Ok(Parsed { func, fmt, pargs })
    }
}
