use bevy_macro_utils::BevyManifest;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Path};
use quote::quote;

extern crate proc_macro;




#[proc_macro_derive(GltfComponent)]
pub fn derive_gltf_component(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generic = &ast.generics;
    let bounds = &generic.where_clause;
    let bevy_gltf_path: Path = bevy_gltf_path();
    TokenStream::from( quote! {
        impl #generic GltfComponent for #name #generic #bounds {}
    })
}


pub(crate) fn bevy_gltf_path() -> syn::Path {
    BevyManifest::default().get_path("bevy_gltf")
}