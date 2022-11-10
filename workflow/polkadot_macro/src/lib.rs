extern crate proc_macro;

use std::collections::HashMap;
use proc_macro::TokenStream;
use quote::*;
use syn::__private::TokenStream2;
use syn::{parse_macro_input, DeriveInput};

mod impl_api_tarits;
use impl_api_tarits::*;
mod impl_imports;
use impl_imports::*;
mod staking_payout;
use staking_payout::*;
mod transfer;
use transfer::*;

#[proc_macro_derive(Polkadot, attributes(Chain, Operation))]
pub fn polkadot_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_polkadot(ast)
}

fn impl_polkadot(ast: DeriveInput) -> TokenStream {
    let polka = ast.ident.clone();
    let attribute_args = ast.attrs.clone();

    let methods = quote! {
        impl #polka{
            pub fn api(&self) -> substrate_api_client::Api<sp_core::sr25519::Pair, WasmRpcClient, BaseExtrinsicParams<PlainTip>>{
                let client = WasmRpcClient::new(self.input.url.clone());

                Api::<sr25519::Pair, _, PlainTipExtrinsicParams>::new(client).unwrap()
            }

            pub fn active_era(&self) -> ActiveEraInfo{
                match self.api().get_storage_value("Staking", "ActiveEra", None).unwrap() {
                    Some(era) =>{// let active_era = era;
                    return era;}
                    None => panic!("Active Era Not Found"),
                }
            }

            pub fn set_signer_by_seed(&self) -> Api<sp_core::sr25519::Pair, WasmRpcClient, BaseExtrinsicParams<PlainTip>>{
                let pair = sr25519::Pair::from_string(&self.input.owner_key, None).unwrap();
                self.api().set_signer(pair)
            }

            pub fn get_account_id(address : &str) -> AccountId32 {
                match AccountId32::from_ss58check(address) {
                    Ok(address) => return address,
                    Err(e) => panic!("Invalid Account id : {:?}", e),
                }
            }

        }

    };

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

    // let chain = property_map["Chain"].clone();
    let operation = property_map["Operation"].clone();

    let mut operation_methods = quote! {};
    if operation.to_lowercase() == "stakingpayout" {
        operation_methods = impl_payout(polka);
    } else if operation.to_lowercase() == "batchedpayout" {
        operation_methods = impl_batched_payout(polka);
    } else if operation.to_lowercase() == "transfer" {
        operation_methods = impl_transfer(polka);
    } else {
    }

    let structs = impl_structs();
    let imports = impl_imports();

    let ast = quote! {
        #methods
        #structs
        #imports
        #operation_methods
    };
    ast.into()
    // methods.into()
}
