use std::{collections::HashMap, marker::PhantomData};

use alloy_primitives::{Address, B256, U256};
use bytes::Bytes;

use crate::{block::BlockHeader, state::{SubState, WorldState}};

pub struct EVM {
    // Global system state (sigma)
    world_state: WorldState,
    // Sub state (A)
    sub_state: SubState,
    // A simple stack with word size = 256 bit. Maximum size of the stack is 1024
    stack: Vec<B256>,
    // A simple word-addressed byte array (address size = 2 bytes, word size = 32 bytes)
    memory: HashMap<u16, B256>,
    // Unlike memory, which is volatile, storage is non volatile and is maintained as part of the system state
    storage: HashMap<B256, B256>,
    // Remaining gas for the execution
    gas: U256,
    // there are several pieces of important information used in the execution environment that the
    // execution agent must provide (I)
    execution_env: ExecutionEnvironment,
}

pub struct ExecutionEnvironment {
    // Ia, the address of the account which owns the code that is executing.
    contract_account: Address,
    // Io, the sender address of the transaction that originated this execution.
    tx_origin: Address,
    // Ip, the price of gas paid by the signer of the transaction that originated this execution. This is defined
    // as the effective gas price p
    gas_price: U256,
    // Id, the byte array that is the input data to this execution; if the execution agent is a transaction,
    // this would be the transaction data.
    input_data: Bytes,
    // Is, the address of the account which caused the code to be executing; if the execution agent is a
    // transaction, this would be the transaction sender.
    sender: Address,
    // Iv, the value, in Wei, passed to this account as part of the same procedure as execution; if the
    // execution agent is a transaction, this would be the transaction value.
    value: U256,
    // Ib, the byte array that is the machine code to be executed
    byte_code: Bytes,
    // IH, the block header of the present block. 
    block_header: BlockHeader,
    // Ie, the depth of the present message-call or contract-creation (i.e. the number of CALLs or
    // CREATE(2)s being executed at present).
    depth: u64,
    // Iw, the permission to make modifications to the state
    permission: bool,
}