use quote::{ToTokens, quote};

use crate::parse::Parsed;

struct Flog(Parsed);
impl ToTokens for Flog {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let this = &self.0;
        let pargs = &this.pargs;
        let func = &this.func;
        let fmt = format!("{}({})", this.fmt_func(), this.fargs.join(", "));
        let fut = if cfg!(feature = "async") {
            quote! {
                use resplus::FutResultChain;
            }
        } else {
            quote! {}
        };
        tokens.extend(quote! {
            {
                use resplus::ResultChain;
                #fut
                #(#pargs)*
                let __res = #func.about_else(move || format!(#fmt));
                __res
            }
        });
    }
}

pub fn flog_impl(parsed: Parsed) -> proc_macro::TokenStream {
    let flog = Flog(parsed);
    quote!(#flog).into()
}
