use std::collections::{HashMap, HashSet};

use alloy_primitives::{Address, B256};
use bytes::Bytes;
use ruint::aliases::U256;

pub struct WorldState {
    // A mapping between addresses (160-bit identifiers) and account state (a data structure serialized as RLP)
    // Though not stored on the blockchain, its assumed that the implementation will maintain this mapping using a
    // modified Merkle Patricia Tree.
    // The trie requires a simple key value database backend that maintains a mapping of bytes array to bytes array.
    // Benefits: the root node of the trie is cryptographically dependent on all internal data so it
    // can be used as a secure identity for the entire system state
    // Secondly, it allows any previous state (whose root hash is known) to be recalled by simply
    // altering the root hash accordingly. Since we store all such root hashes in the blockchain, we are able to
    // trivially revert to old states.
    trie: HashMap<Address,  AccountState>,
}

pub struct AccountState {
    // Number of transactions sent from this address or 
    // in the case of contract-account, the number of contract-creation made by this account
    nonce: U256,
    // Number of Wei owned by this address
    balance: U256,
    // A 256 bit hash of the root node of the trie that encode the storage content of an account
    // (a mapping between 256 bit integer values)
    // encoded into the trie as: keccak256(all keys) -> RLP encoded of all values
    // Note: storage_root is not a "physical" member of the AccountState and does not contribute to it serialization
    storage_root: B256,
    // The hash of the EVM byte codes of this address
    code_hash: B256,
    // code: if None, do nothing otherwise load the code from storage if it need to execute calls
    code: Option<Bytes>
}

pub struct SubState {
    // Accrued information that is acted upon immediately following the transaction
    // A_s: set of accounts that will be discarded following the transaction completion
    self_destruct_set: HashSet<Address>,
    // A_l: the log series: series of archived and indexable 'checkpoints' in VM code execution
    // that allow contract-calls to be easily tracked by onlooker external to the Ethereum world
    logs: Vec<Bytes>,
    // A_t: set of touched accounts, the empty ones are deleted after transaction end
    touched_accounts: HashSet<Address>,
    // A_r: refund balance, increased through using the SSTORE instruction to reset contract storage
    refund: B256,
    // EIP-2929, A_a: set of accessed account addresses, A_k, set of accessed storage keys
    // A_k = (address, 32-byte storage slot) why 32-byte? Because 32-byte = 256 bit
    // Note: In the Yellow Paper, A_a is initialized as Pi, set of precompiled addresses, we'll ignore them for now
    // since we don't even know what the heck they are
    accessed_accounts: HashSet<Address>,
    accessed_storage: HashMap<Address, Bytes>,
}

// Machine state (µ)
// we define it recursively with a function X. This uses an iterator function O (which defines the result of a
// single cycle of the state machine) together with functions Z which determines if the present state is an exceptional
// halting state of the machine and H, specifying the output data of the instruction if and only if the present state is a
// normal halting state of the machine
// Note: emmpty sequence ([]) is different with empty set (())
// empty set -> continue, empty sequence -> halt
pub struct EVMState {
    // gas available
    gas: u64,
    // program counter
    pc: U256,
    // memory contents: a series of zeroes of size 2^256
    // it's a series, so we use Vec, but no computer can allocate Vec size of 2^256 (no program ever need that fucking memory)
    // so I think we'll be fine
    m: Vec<B256>,
    // active number of words in memory
    i: usize,
    // stack contents
    s: Vec<B256>,
    // returndata buffer
    o: Bytes
}