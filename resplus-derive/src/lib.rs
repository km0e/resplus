extern crate proc_macro;

use core::panic;

use quote::{ToTokens, format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token, parenthesized};

use proc_macro2::TokenStream as TokenStream2;

/// receive expr like func(@a,b,@c,d)
#[proc_macro]
pub fn flog(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as ParsedMap);
    quote!(#input).into()
}

#[derive(Debug, Clone)]
struct IndexArg {
    idx: usize,
}

impl From<usize> for IndexArg {
    fn from(idx: usize) -> Self {
        IndexArg { idx }
    }
}

impl ToTokens for IndexArg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let idx = self.idx;
        let idx = format_ident!("_{}", idx);
        tokens.extend(quote! {
            #idx
        });
    }
}

struct PrintArg {
    idx: IndexArg,
    arg: Expr,
}

impl PrintArg {
    fn new(idx: impl Into<IndexArg>, arg: Expr) -> Self {
        PrintArg {
            idx: idx.into(),
            arg,
        }
    }
}

#[derive(Clone)]
enum Arg {
    Regular(Expr),
    Indexed(IndexArg),
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Arg::Regular(expr) => {
                tokens.extend(quote! {
                    #expr
                });
            }
            Arg::Indexed(idx) => {
                tokens.extend(quote! {
                    #idx
                });
            }
        }
    }
}

impl ToTokens for PrintArg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let idx = self.idx.clone();
        let arg = self.arg.clone();
        tokens.extend(quote! {
            let #idx = #arg;
        });
    }
}

struct FormatFunc {
    fmt: String,
    max_idx: usize,
}

impl ToTokens for FormatFunc {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let fmt = self.fmt.clone();
        if self.max_idx > 0 {
            let mut idxs = Vec::new();
            for i in 0..self.max_idx {
                idxs.push(IndexArg { idx: i });
            }
            tokens.extend(quote! {
                format!(#fmt #(,#idxs)*)
            });
        } else {
            tokens.extend(quote! {
                #fmt
            });
        }
    }
}

struct ParsedMap {
    fmt: FormatFunc,
    func: syn::Ident,
    pargs: Vec<PrintArg>,
    args: Vec<Arg>,
}

impl ToTokens for ParsedMap {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pargs = &self.pargs;
        let func = &self.func;
        let args = &self.args;
        let fmt = &self.fmt;
        tokens.extend(quote! {
            {
                use resplus::ResultChain;
                #(#pargs)*
                #func(#(#args),*).about_else(|| #fmt)?
            }
        });
    }
}

struct TaggedValue {
    tag: bool,
    value: Expr,
}

impl Parse for TaggedValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag = if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            true
        } else {
            false
        };
        let value = input.parse::<syn::Expr>()?;
        Ok(TaggedValue { tag, value })
    }
}

impl Parse for ParsedMap {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            panic!("There should be a function call");
        }
        let func = input.parse::<syn::Ident>().expect("no function name");
        let mut pargs = Vec::new();
        let mut fargs = Vec::new();
        let mut args = Vec::new();
        let content;
        parenthesized!(content in input);
        if !content.is_empty() {
            let mut idx = 0;
            loop {
                let tagged_value = content.parse::<TaggedValue>()?;
                if tagged_value.tag {
                    pargs.push(PrintArg::new(idx, tagged_value.value));
                    args.push(Arg::Indexed(idx.into()));
                    idx += 1;
                    fargs.push("{}");
                } else {
                    args.push(Arg::Regular(tagged_value.value));
                    fargs.push("_");
                }
                if content.is_empty() {
                    break;
                }
                content.parse::<Token![,]>()?;
            }
        };
        let fmt = format!("{}({})", func.to_token_stream(), fargs.join(", "));
        Ok(ParsedMap {
            func,
            fmt: FormatFunc {
                fmt,
                max_idx: pargs.len(),
            },
            args,
            pargs,
        })
    }
}
