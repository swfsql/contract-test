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
        struct Input {
            message: String,
        }
        impl borsh::ser::BorshSerialize for Input
        where
            String: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.message, writer)?;
                Ok(())
            }
        }
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
