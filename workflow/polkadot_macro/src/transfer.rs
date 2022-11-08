use super::*;
use syn::Ident;
pub fn impl_transfer(struct_name: Ident) -> TokenStream2 {

    let transfer_methods = quote!{
        impl #struct_name{
            pub fn transfer(&self)-> Option<H256>{
                let api = self.set_signer_by_seed();
                let account = #struct_name::get_account_id(&self.op_values.address);
                let call = api.balance_transfer(MultiAddress::Id(account.clone()), self.op_values.amount.into());
                let result = api.send_extrinsic(call.hex_encode()).unwrap();

                result
            }
        }
    };

    transfer_methods
}