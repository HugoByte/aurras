use super::*;

#[cfg(not(tarpaulin_include))]
pub fn impl_structs() -> TokenStream2 {
    let structs = quote! {
    #[derive(Serialize, Deserialize)]
    pub struct RpcResult {
        pub result: Box<RawValue>,
    }

    #[cfg(target_arch = "wasm32")]
    use openwhisk_rust::WasmClient;

    #[cfg(target_arch = "wasm32")]
    pub struct WasmRpcClient {
        url: String,
        client: WasmClient,
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub struct WasmRpcClient {
        url: String,
        client: NativeClient,
    }

    #[cfg(not(target_arch = "wasm32"))]
    use openwhisk_rust::NativeClient;

    #[cfg(target_arch = "wasm32")]
    impl WasmRpcClient {
        pub fn new(url: String) -> Self {
            Self {
                url,
                client: WasmClient::new_whisk_client(Some(false)),
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl WasmRpcClient {
        pub fn new(url: String) -> Self {
            Self {
                url,
                client: NativeClient::new_whisk_client(Some(false)),
            }
        }
    }

    impl WasmRpcClient {

        pub fn request(&self, body: serde_json::Value) -> substrate_api_client::ApiResult<String> {
            let request = self
                .client
                .new_request(Some(HttpMethods::POST), &self.url, None, Some(body))
                .map_err(|err| substrate_api_client::std::error::Error::RpcClient(err))?;

            self.client
                .invoke_request(request)
                .map(|value| {
                    return serde_json::from_value::<RpcResult>(value)
                        .unwrap()
                        .result
                        .to_string();
                })
                .map_err(|err| return substrate_api_client::std::error::Error::RpcClient(err))
        }
    }

    impl RpcClient for WasmRpcClient {
        fn get_request(&self, body: serde_json::Value) -> substrate_api_client::ApiResult<String> {
            self.request(body)
        }

        fn send_extrinsic(
            &self,
            xthex_prefixed: String,
            exit_on: XtStatus,
        ) -> substrate_api_client::ApiResult<Option<sp_core::H256>> {
            let jsonreq = match exit_on {
                XtStatus::SubmitOnly => json_req::author_submit_extrinsic(&xthex_prefixed),
                _ => json_req::author_submit_extrinsic(&xthex_prefixed),
            };

            self.request(jsonreq)
                .map(|response| Some(sp_core::H256::from_hex(response).unwrap()))
        }
    }
    };
    structs
}
