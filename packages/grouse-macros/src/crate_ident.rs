use std::cell::OnceCell;

use quote::{ToTokens, quote};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrateIdent;

impl ToTokens for CrateIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        thread_local! {
            static TOKENS: OnceCell<proc_macro2::TokenStream> = OnceCell::new();
        }

        TOKENS.with(|x| {
            x.get_or_init(|| match proc_macro_crate::crate_name("grouse").unwrap() {
                proc_macro_crate::FoundCrate::Itself => quote! { crate },
                proc_macro_crate::FoundCrate::Name(ident) => {
                    let ident = proc_macro2::Ident::new(&ident, proc_macro2::Span::call_site());

                    quote! { ::#ident }
                }
            })
            .to_tokens(tokens);
        });
    }
}
