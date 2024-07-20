use alloy_primitives::{Address, Bytes, U256};

use crate::{
    block::BlockHeader,
    functions::FUNCTIONS,
    operations::{OpCode, OPERATIONS},
    state::{EVMState, SubState, WorldState},
    transaction::Transaction,
};

pub struct EVM<'a> {
    // Global system state (sigma)
    pub world_state: &'a mut WorldState,
    // Sub state (A)
    pub sub_state: &'a mut SubState,
    // there are several pieces of important information used in the execution environment that the
    // execution agent must provide (I)
    pub execution_env: &'a ExecutionEnvironment<'a>,
    // Machine state (µ: mu)
    pub state: EVMState,
}

impl<'a> EVM<'a> {
    pub fn new(
        world_state: &'a mut WorldState,
        sub_state: &'a mut SubState,
        execution_env: &'a ExecutionEnvironment,
    ) -> Self {
        EVM {
            world_state,
            sub_state,
            execution_env,
            state: EVMState::new(),
        }
    }

    pub fn execute(&mut self) {
        while self.state.pc < self.execution_env.byte_code.len() {
            let b = self.execution_env.byte_code[self.state.pc];
            let operation = &OPERATIONS[b as usize];
            if operation.name == OpCode::STOP {
                return;
            }
            let func = &FUNCTIONS[b as usize];
            func(operation, self);
        }
    }

    pub fn execute_tx(&mut self, tx: &Transaction) {}
}

pub struct ExecutionEnvironment<'a> {
    // Ia, the address of the account which owns the code that is executing.
    pub contract_account: Address,
    // Io, the sender address of the transaction that originated this execution.
    pub tx_origin: Address,
    // Ip, the price of gas paid by the signer of the transaction that originated this execution. This is defined
    // as the effective gas price p
    pub gas_price: U256,
    // Id, the byte array that is the input data to this execution; if the execution agent is a transaction,
    // this would be the transaction data.
    pub input_data: Bytes,
    // Is, the address of the account which caused the code to be executing; if the execution agent is a
    // transaction, this would be the transaction sender.
    pub sender: Address,
    // Iv, the value, in Wei, passed to this account as part of the same procedure as execution; if the
    // execution agent is a transaction, this would be the transaction value.
    pub value: U256,
    // Ib, the byte array that is the machine code to be executed
    pub byte_code: Bytes,
    // IH, the block header of the present block.
    pub block_header: &'a BlockHeader,
    // Ie, the depth of the present message-call or contract-creation (i.e. the number of CALLs or
    // CREATE(2)s being executed at present).
    pub depth: u64,
    // Iw, the permission to make modifications to the state
    pub permission: bool,
}

impl<'a> ExecutionEnvironment<'a> {
    pub fn new(
        contract_account: Address,
        tx_origin: Address,
        gas_price: U256,
        input_data: Bytes,
        sender: Address,
        value: U256,
        byte_code: Bytes,
        block_header: &'a BlockHeader,
        depth: u64,
        permission: bool,
    ) -> Self {
        ExecutionEnvironment {
            contract_account,
            tx_origin,
            gas_price,
            input_data,
            sender,
            value,
            byte_code,
            block_header,
            depth,
            permission,
        }
    }
}
