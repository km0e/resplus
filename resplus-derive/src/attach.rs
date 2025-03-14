use quote::{ToTokens, quote};

use crate::parse::Parsed;

struct Attach(Parsed);
impl ToTokens for Attach {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let this = &self.0;
        let pargs = &this.pargs;
        let func = &this.func;
        let fmt = format!("{}({})", this.fmt_func(), this.fargs.join(", "));
        let fut = if cfg!(feature = "async") {
            quote! {
                use crate::error::FutResultChainAttach;
            }
        } else {
            quote! {}
        };
        tokens.extend(quote! {
            {
                use crate::error::ResultChainAttach;
                #fut
                #(#pargs)*
                let __res = #func.attach_else(move || format!(#fmt));
                __res
            }
        });
    }
}

pub fn attach_impl(parsed: Parsed) -> proc_macro::TokenStream {
    let flog = Attach(parsed);
    quote!(#flog).into()
}
