use crate_ident::CrateIdent;
use quote::quote;

mod cache;
mod crate_ident;
mod digest;
mod manifest;

#[proc_macro]
pub fn digest(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = syn::parse_macro_input!(item as syn::LitStr);

    match manifest::digest(&path) {
        Ok(Some(digest)) => {
            let digest = digest.to_hex();

            quote! { #digest }.into()
        }

        Ok(None) => quote! { "<unresolved>" }.into(),

        Err(err) => syn::Error::new(path.span(), err)
            .into_compile_error()
            .into(),
    }
}

#[proc_macro]
pub fn include(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = syn::parse_macro_input!(item as syn::LitStr);

    match manifest::bundle(&path) {
        Ok(Some(manifest)) => {
            let files = manifest.entries().map(|x| {
                let path = x.path().to_str().unwrap();
                let name = x.path().file_name().unwrap().to_str().unwrap();

                let digest = x.digest().to_hex();

                let mime = mime_guess::from_path(x.path())
                    .first_or_octet_stream()
                    .to_string();

                quote! {
                    #CrateIdent::File {
                        bytes: ::core::include_bytes!(#path),
                        name: #name,
                        digest: #digest,
                        mime: #mime,
                    }
                }
            });

            let index = manifest.entries().enumerate().map(|(index, x)| {
                let digest = x.digest().to_hex();

                quote! {
                    #digest => ::core::option::Option::Some(unsafe { FILES.get_unchecked(#index) })
                }
            });

            quote! {{
                const FILES: &'static [#CrateIdent::File] = &[#(#files),*];

                #CrateIdent::Manifest {
                    files: FILES,
                    index: |x| match x {
                        #(#index,)*

                        _ => ::core::option::Option::None,
                    }
                }
            }}
            .into()
        }

        Ok(None) => quote! { "<unresolved>" }.into(),

        Err(err) => syn::Error::new(path.span(), err)
            .into_compile_error()
            .into(),
    }
}
