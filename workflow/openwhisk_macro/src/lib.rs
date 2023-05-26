extern crate proc_macro;
extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::HashMap;

use quote::*;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[cfg(not(tarpaulin_include))]
#[proc_macro_derive(OpenWhisk, attributes(AuthKey, ApiHost, Insecure, Namespace))]
pub fn client(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_openwhisk_client(ast)
}

#[cfg(not(tarpaulin_include))]
fn impl_openwhisk_client(ast: DeriveInput) -> TokenStream {
    let name = ast.ident.clone();
    let attribute_args = ast.attrs;

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

        for segment in path.segments {
            property_map.insert(segment.ident.to_string(), value.clone());
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
            pub fn openwhisk_client(&self) -> OpenwhiskClient<WasmClient>{
                let wskprops  = WskProperties::new(#auth_token.to_string(), #api_host.to_string(), #insecure, #namespace.to_string());
                OpenwhiskClient::<WasmClient>::new(Some(&wskprops))
            }

            pub fn run(&mut self) -> Result<(),String>{
                let payload = serde_json::to_value(self.input.clone()).map_err(|e|e.to_string())?;
                
                let result = self.openwhisk_client()
                        .actions()
                        .invoke(&self.action_name, payload, true, true)?;
            
                
                self.output = serde_json::from_value(result).map_err(|e|e.to_string())?;
                Ok(())
            }

            fn action_name(&self) -> &str {
                self.action_name.as_str()
            }
        }
    };
    impl_whisk_client.into()
}
