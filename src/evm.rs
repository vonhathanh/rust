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

    pub fn execute_tx(&mut self, tx: &Transaction) -> ExecutionStatus {
        if !self.validate_tx(tx) {
            Error::TransactionIsNotValid;
        }
        // increase nonce of the sender by one (irrevocable)
        // balance -= part of the up-front cost Tgp
        // gas = Tg - g0
        // define the checkpoint state sigma_o by equaltion (68-70)
        // calculate tuple of post-execution provisional states =
        // (sigma_p, remaining gas g', accrued substate A, status code z) in eual (71)
        // determine the refund amount g*
        // update the final state sigma_*: subtract the refund from gas cost, add ether to block submitter
    }

    fn validate_tx(&self, tx: &Transaction) -> bool {
        // It's assumed that any transactions executed first pass the initial tests of intrinsic validity.
        // 1. The transaction is well-formed RLP, with no additional trailing bytes
        // 2. the transaction signature is valid
        // 3. the transaction nonce is valid (equivalent to the sender account’s current nonce)
        // 4. the sender account has no contract code deployed
        // 5. the gas limit is no smaller than the intrinsic gas, g0 , used by the transaction
        // 6. the sender account balance contains at least the cost, v0 , required in up-front payment
        // 7. the maxFeePerGas, Tm , in the case of type 2 transactions, or gasPrice, Tp , in the case of type
        // 0 and type 1 transactions, is greater than or equal to the block’s base fee, Hf ; and
        // 8. for type 2 transactions, maxPriorityFeePerGas, Tf , must be no larger than maxFeePerGas, Tm

        // Implementation
        // calculate g0, the amount of gas this tx requires to be paid prior to excution
        // get effective gas price p: the amount of wei the transaction signer will pay per unit of
        // gas consumed during the transaction’s execution
        // calculate the up-front cost vo
        // check validity by equaltion (65)
        true
    }
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
