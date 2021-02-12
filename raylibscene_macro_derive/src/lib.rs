extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ClientGetter)]
pub fn raylibscene_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_raylibscene_derive(&ast)
}

fn impl_raylibscene_derive(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let generated = quote! {
        impl ClientGetter for #struct_name {
            fn client(&self) -> Option<Arc<HangmanClient>> {
                Some(Arc::clone(&self.client))
            }
        }
    };

    generated.into()
}
