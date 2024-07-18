use std::ops::Add;

use ruint::aliases::B256;

use crate::{evm::EVM, operations::Operation};

pub fn add(op: &Operation, evm: &mut EVM) {
    let s1 = evm.state.s.pop();
    let s2 = evm.state.s.pop();
    let result = s1.unwrap().as_uint().add(s2.unwrap().as_uint());
    evm.state.s.push(B256::from(result));
}