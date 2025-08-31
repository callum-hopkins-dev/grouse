use std::cell::OnceCell;

use quote::ToTokens;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrateIdent;

impl ToTokens for CrateIdent {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        thread_local! {
            static CRATE_NAME: OnceCell<Box<str>> = OnceCell::new();
        }

        CRATE_NAME.with(|crate_name| {
            let crate_name =
                crate_name.get_or_init(|| match proc_macro_crate::crate_name("grouse").unwrap() {
                    proc_macro_crate::FoundCrate::Itself => "grouse".into(),

                    proc_macro_crate::FoundCrate::Name(x) => x.into_boxed_str(),
                });

            proc_macro2::Ident::new(&crate_name, proc_macro2::Span::call_site()).to_tokens(tokens);
        });
    }
}
