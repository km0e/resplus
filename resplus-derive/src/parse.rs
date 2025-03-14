use std::borrow::Cow;

use quote::{ToTokens, quote};
use syn::{
    Expr, ExprCall, ExprLit, ExprMethodCall, ExprRange, Lit, LitInt, Token,
    parse::{Parse, ParseStream},
    parse_str,
    spanned::Spanned,
};
pub struct TmpArg {
    idx: Expr,
    arg: Expr,
}

impl TmpArg {
    fn new(idx: Expr, arg: Expr) -> Self {
        TmpArg { idx, arg }
    }
}

impl ToTokens for TmpArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let idx = &self.idx;
        let arg = &self.arg;
        tokens.extend(quote! {
            let #idx = #arg;
        });
    }
}
pub struct Parsed {
    pub fargs: Vec<Cow<'static, str>>,
    pub func: Expr,
    pub pargs: Vec<TmpArg>,
}

impl Parsed {
    pub fn fmt_func(&self) -> String {
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
//
// impl ToTokens for Parsed {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         let pargs = &self.pargs;
//         let func = &self.func;
//         let fmt = format!("{}({})", self.fmt_func(), self.fargs.join(", "));
//         let fut = if cfg!(feature = "async") {
//             quote! {
//                 use resplus::FutResultChain;
//             }
//         } else {
//             quote! {}
//         };
//         tokens.extend(quote! {
//             {
//                 use resplus::ResultChain;
//                 #fut
//                 #(#pargs)*
//                 let __res = #func.about_else(move || format!(#fmt));
//                 __res
//             }
//         });
//     }
// }
//
impl Parse for Parsed {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let boxed_expr2usize = |e: Option<Box<Expr>>, default: usize| -> syn::Result<usize> {
            let u = e
                .map(|s| {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Int(lit), ..
                    }) = s.as_ref()
                    {
                        lit.base10_parse::<usize>()
                    } else {
                        Err(syn::Error::new(s.span(), "expected integer"))
                    }
                })
                .transpose()?
                .unwrap_or(default);
            Ok(u)
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
                let start = boxed_expr2usize(range.start, 0)?;
                let end = boxed_expr2usize(range.end, args.len())?;
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
                pargs.push(TmpArg::new(
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
        Ok(Parsed { fargs, func, pargs })
    }
}
