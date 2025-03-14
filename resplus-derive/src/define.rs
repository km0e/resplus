use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Ident, Token, Type, parse_macro_input,
    punctuated::{Pair, Punctuated},
};

pub fn define_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut p = parse_macro_input!(input with Punctuated::<Type, Token![,]>::parse_terminated);
    if p.empty_or_trailing() {
        panic!("expected at least one type");
    }
    let Pair::End(target) = p.pop().unwrap() else {
        panic!("expected not trailing comma");
    };
    let p = p.into_iter();

    let source_cast = quote! {
        #(
            impl From<#p> for ErrorChain {
                fn from(value: #p) -> Self {
                    ErrorChain(resplus::ErrorChain::new(value))
                }
            }
        )*
    };

    let need = vec![TokenStream::new()];
    let need = &need;
    let no_need = Vec::<TokenStream>::new();
    let no_need = &no_need;

    let trait_define = |trait_ident: Ident, is_async: &Vec<TokenStream>| {
        quote! {
            #(#is_async #[async_trait::async_trait])*
            pub trait #trait_ident<T, D>
                where
                    Self: Sized,
                    D: Into<std::borrow::Cow<'static, str>>,
                {
                #(#is_async async)* fn attach(self, desc: D) -> Result<T, ErrorChain>;
                #(#is_async async)*fn attach_else(self, f: impl Send + FnOnce() -> D) -> Result<T, ErrorChain>;
            }
        }
    };
    let trait_fn = |str_type: TokenStream, is_async: &Vec<TokenStream>| {
        quote! {
            #(#is_async async)* fn attach(self, desc: #str_type) -> Result<T, ErrorChain>{
                self #(#is_async.await)*.map_err(|mut e| {
                    e.0.context.push(desc.into());
                    e
                })
            }
            #(#is_async async)*fn attach_else(self, f: impl Send + FnOnce() -> #str_type) -> Result<T, ErrorChain>{
                self #(#is_async.await)*.map_err(|mut e| {
                    e.0.context.push(f().into());
                    e
                })
            }
        }
    };
    let trait_impl = |trait_ident: Ident, str_type: TokenStream, is_async: &Vec<TokenStream>| {
        let tfn = trait_fn(str_type.clone(), is_async);
        quote! {
            #(#is_async #[async_trait::async_trait])*
            impl<T> #trait_ident<T, #str_type> for std::result::Result<T, ErrorChain>
            where
                Self: Sized,
            {
                #tfn
            }
        }
    };
    let async_trait_impl =
        |trait_ident: Ident, str_type: TokenStream, is_async: &Vec<TokenStream>| {
            let tfn = trait_fn(str_type.clone(), is_async);
            quote! {
                #(#is_async #[async_trait::async_trait])*
                impl<T,F> #trait_ident<T, #str_type> for F
                where
                    Self: Sized,
                    F: Future<Output = std::result::Result<T, ErrorChain>> + Send,
                {
                    #tfn
                }
            }
        };
    let target_cast = quote! {
        impl From<#target> for ErrorChain {
            fn from(value: #target) -> Self {
                ErrorChain(resplus::ErrorChain::new(value))
            }
        }

        impl From<resplus::ErrorChain<#target>> for ErrorChain {
            fn from(value: resplus::ErrorChain<#target>) -> Self {
                ErrorChain(value)
            }
        }

        #[derive(Debug)]
        pub struct ErrorChain(resplus::ErrorChain<#target>);

        impl std::fmt::Display for ErrorChain {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::error::Error for ErrorChain {}
    };
    let static_str = quote! {
        &'static str
    };
    let string = quote! {
        String
    };
    let default_trait = trait_define(Ident::new("ResultChainAttach", Span::call_site()), no_need);
    let default_trait_str_impl = trait_impl(
        Ident::new("ResultChainAttach", Span::call_site()),
        static_str.clone(),
        no_need,
    );
    let default_trait_string_impl = trait_impl(
        Ident::new("ResultChainAttach", Span::call_site()),
        string.clone(),
        no_need,
    );
    let async_trait = trait_define(Ident::new("FutResultChainAttach", Span::call_site()), need);
    let async_trait_str_impl = async_trait_impl(
        Ident::new("FutResultChainAttach", Span::call_site()),
        static_str,
        need,
    );
    let async_trait_string_impl = async_trait_impl(
        Ident::new("FutResultChainAttach", Span::call_site()),
        string,
        need,
    );

    quote! {
        #source_cast
        #target_cast
        #default_trait
        #default_trait_str_impl
        #default_trait_string_impl
        #async_trait
        #async_trait_str_impl
        #async_trait_string_impl
    }
    .into()
}
