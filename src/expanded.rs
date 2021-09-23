#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use near_sdk::borsh;
use near_sdk::ext_contract;
pub mod message {
    use super::*;
    use near_sdk::{Gas, Balance, AccountId, Promise};
    pub fn method_a(
        message: String,
        __account_id: AccountId,
        __balance: near_sdk::Balance,
        __gas: near_sdk::Gas,
    ) -> near_sdk::Promise {
        #[serde(crate = "near_sdk::serde")]
        struct Input {
            message: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            use near_sdk::serde as _serde;
            #[automatically_derived]
            impl near_sdk::serde::Serialize for Input {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> near_sdk::serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: near_sdk::serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Input",
                        false as usize + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "message",
                        &self.message,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        let args = Input { message };
        let args = near_sdk::serde_json::to_vec(&args)
            .expect("Failed to serialize the cross contract args using JSON.");
        near_sdk::Promise::new(__account_id).function_call(
            "method_a".to_string(),
            args,
            __balance,
            __gas,
        )
    }
}
