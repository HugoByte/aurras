use super::*;

#[cfg(not(tarpaulin_include))]
pub fn impl_imports() -> TokenStream2{
    let imports = quote!{
        use codec::{Decode, Encode};
        use openwhisk_rust::{HttpMethods, Service};
        use serde_json::Value;
        use sp_core::crypto::Ss58Codec;
        use sp_core::{sr25519, Pair};
        use sp_runtime::AccountId32;
        use substrate_api_client::{Api, PlainTipExtrinsicParams, RpcClient, XtStatus};
        use substrate_api_client::{BaseExtrinsicParams, PlainTip};
        use openwhisk_rust::OpenWhisk;
        use pallet_staking::{ActiveEraInfo, Exposure};
        use serde_json::value::RawValue;
        use substrate_api_client::rpc::json_req;
        use substrate_api_client::{FromHexString,SubstrateDefaultSignedExtra,UncheckedExtrinsicV4,AccountDataGen,ApiClientError};
    };
    imports
}