use near_sdk::borsh;
use near_sdk::ext_contract;

#[ext_contract]
pub trait Message {
    #[result_serializer(borsh)]
    fn method_a(
        &mut self, 
        #[serializer(borsh)]
        message: String,
    );
}

