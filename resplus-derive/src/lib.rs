extern crate proc_macro;

use core::panic;
use std::borrow::Cow;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    Expr, ExprCall, ExprLit, ExprMethodCall, ExprRange, Lit, LitInt, Token, parse_macro_input,
    parse_str,
};

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
    fargs: Vec<Cow<'static, str>>,
    func: Expr,
    pargs: Vec<PrintArg>,
}

impl Parsed {
    fn fmt_func(&self) -> String {
        match &self.func {
            Expr::Call(ExprCall { func, .. }) => func.to_token_stream().to_string(),
            Expr::MethodCall(ExprMethodCall { method, .. }) => method.to_string(),
            _ => panic!(
                "{}",
                syn::Error::new(self.func.span(), "expected function call")
            ),
        }
    }
}

impl ToTokens for Parsed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pargs = &self.pargs;
        let func = &self.func;
        let fmt = format!("{}({})", self.fmt_func(), self.fargs.join(", "));
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
        let boxed_expr2usize = |e: Option<Box<Expr>>| {
            e.map(|s| {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Int(lit), ..
                }) = s.as_ref()
                {
                    return lit.base10_parse::<usize>();
                }
                Err(syn::Error::new(s.span(), "expected integer"))
            })
            .transpose()
        };
        if input.is_empty() {
            panic!("There should be a function call");
        }
        let mut func = input.parse::<Expr>()?;
        let args = match &mut func {
            Expr::Call(ExprCall { args, .. }) => args,
            Expr::MethodCall(ExprMethodCall { args, .. }) => args,
            _ => Err(syn::Error::new(func.span(), "expected function call"))?,
        };
        let mut fargs = Vec::new();
        let mut pargs = Vec::new();

        let mut skip_idx = 0;
        while input.parse::<Token![,]>().is_ok() {
            let range = if let Ok(range) = input.fork().parse::<ExprRange>() {
                input.parse::<ExprRange>()?;
                let start = boxed_expr2usize(range.start)?.unwrap_or(0);
                let end = boxed_expr2usize(range.end)?.unwrap_or(args.len());
                start..end
            } else if let Ok(idx) = input.parse::<LitInt>() {
                let idx = idx.base10_parse::<usize>()?;
                idx..idx + 1
            } else {
                Err(syn::Error::new(input.span(), "expected integer or range"))?;
                break;
            };
            while skip_idx < range.start {
                fargs.push(Cow::Borrowed("_"));
                skip_idx += 1;
            }
            skip_idx = range.end;
            for i in range {
                let tmp_id_str = format!("__{}", i);
                let tmp_id = parse_str::<Expr>(&tmp_id_str).unwrap();
                pargs.push(PrintArg::new(
                    tmp_id.clone(),
                    std::mem::replace(&mut args[i], tmp_id),
                ));
                fargs.push(format!("{{{tmp_id_str}}}").into());
            }
        }
        while skip_idx < args.len() {
            fargs.push("_".into());
            skip_idx += 1;
        }

        // let fmt = format!("{}({})", func.func.to_token_stream(), fargs.join(", "));
        // Ok(Parsed { func, fmt, pargs })
        Ok(Parsed { fargs, func, pargs })
    }
}
