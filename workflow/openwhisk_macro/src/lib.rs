extern crate proc_macro;
extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::HashMap;

use quote::*;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(OpenWhisk, attributes(AuthKey, ApiHost, Insecure, Namespace))]
pub fn client(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_openwhisk_client(ast)
}

fn impl_openwhisk_client(ast: DeriveInput) -> TokenStream {
    let name = ast.ident.clone();
    let attribute_args = ast.attrs.clone();

    let mut property_map: HashMap<String, String> = HashMap::new();

    for attribute in attribute_args.into_iter() {
        let (path, value) = match attribute.parse_meta().unwrap() {
            syn::Meta::NameValue(syn::MetaNameValue {
                path,
                lit: syn::Lit::Str(s),
                ..
            }) => (path, s.value()),
            _ => (syn::Path::into(attribute.path), "".to_string()),
        };

        for seg in path.segments {
            property_map.insert(seg.ident.to_string().to_string(), value.clone());
        }
    }

    let insecure = match property_map["Insecure"].as_str() {
        "true" => true,
        "false" => false,
        _ => false,
    };

    let auth_token = property_map["AuthKey"].clone();
    let api_host = property_map["ApiHost"].clone();
    let insecure = insecure;
    let namespace = property_map["Namespace"].clone();

    let impl_whisk_client = quote! {

        impl #name {

            pub fn openwhisk_client(&self) -> OpenwhiskClient<NativeClient>{

                let wskprops  = WskProperties::new(#auth_token.to_string(), #api_host.to_string(), #insecure, #namespace.to_string());

                OpenwhiskClient::<NativeClient>::new(Some(&wskprops))
            }
        }
    };
    impl_whisk_client.into()
}
