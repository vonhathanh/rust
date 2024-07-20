use alloy_primitives::{hex::FromHex, Address, B256, U256};
use block::BlockHeader;
use evm::{ExecutionEnvironment, EVM};
use state::{EVMState, SubState, WorldState};
use transaction::Transaction;

mod block;
mod evm;
mod functions;
mod operations;
mod state;
mod transaction;

fn main() {
    let mut world_state = WorldState::new();
    let mut evm_state = EVMState::new();
    let mut accrued_substate = SubState::new();
    let mut block_header = BlockHeader::new();
    let default_sender = Address::from_hex("0xA160cdAB225685dA1d56aa342Ad8841c3b53f291").unwrap();
    let tx = Transaction::new(
        0,
        U256::from(1),
        100000,
        Some(Address::from_hex("0x6148ce093dcbd629cfbc4203c18210567d186c66").unwrap()),
        U256::ZERO,
        B256::ZERO,
        B256::ZERO,
        None,
        Some(1),
        Some(1),
        None,
        None,
        None,
        Some(U256::from(1000)),
        None,
        None,
    );

    let execute_env = ExecutionEnvironment::new(
        tx.to.unwrap(),
        default_sender,
        tx.gas_price.unwrap(),
        tx.data.unwrap(),
        default_sender,
        tx.value,
        world_state.trie.get(&tx.to.unwrap()).unwrap().code.unwrap(),
        &block_header,
        1,
        true,
    );

    let mut evm = EVM::new(&mut world_state, &mut accrued_substate, &execute_env);
    evm.execute_tx(&tx);
}
