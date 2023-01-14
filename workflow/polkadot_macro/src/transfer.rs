use super::*;
use syn::Ident;
///Implement the transfer method for the struct
pub fn impl_transfer(struct_name: Ident) -> TokenStream2 {
    let transfer_methods = quote! {
        impl #struct_name{
            /// The transfer method will transfer the amount from owner to the given address.
            pub fn transfer(&self)-> Option<H256>{
                let api = self.set_signer_by_seed();
                let account = #struct_name::get_account_id(&self.input.address);
                let call = api.balance_transfer(MultiAddress::Id(account.clone()), self.input.amount.into());
                let result = api.send_extrinsic(call.hex_encode()).unwrap();

                result
            }
            pub fn run(&mut self) -> Result<(), String> {
                let result = self.transfer();
                self.output =  serde_json::json!({"result": result});
                Ok(())
            }
        }
    };

    transfer_methods
}
