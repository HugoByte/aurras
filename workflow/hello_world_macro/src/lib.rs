extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(HelloWorldDerive)]
pub fn hello_world_derive_macro(item: TokenStream) -> TokenStream {

    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;

    quote::quote! {
        impl #ident{
            pub fn run(&mut self) -> Result<(), String>{
                self.output = serde_json::to_value(format!("Hello {}", self.input.name))
                    .map_err(|e|e.to_string())?;
                Ok(())
            }
        }
    }
    .into()
}
