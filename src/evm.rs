use std::collections::HashMap;

use alloy_primitives::{B256, U256};

use crate::state::{SubState, WorldState};

pub struct EVM {
    // Global system state
    world_state: WorldState,
    // Sub state
    sub_state: SubState,
    // A simple stack with word size = 256 bit. Maximum size of the stack is 1024
    stack: Vec<B256>,
    // A simple word-addressed byte array (address size = 2 bytes, word size = 32 bytes)
    memory: Vec<B256>,
    // Unlike memory, which is volatile, storage is non volatile and is maintained as part of the system state
    storage: HashMap<B256, B256>,
    // Remaining gas for the execution
    gas: U256,
    // execution_context: Context,
}