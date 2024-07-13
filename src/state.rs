use std::collections::HashMap;

use bytes::Bytes;

use crate::types::{B160, B256};

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
    trie: HashMap<B160,  AccountState>,
}
pub struct EVMState;

pub struct AccountState {
    // Number of transactions sent from this address or 
    // in the case of contract-account, the number of contract-creation made by this account
    nonce: B256,
    // Number of Wei owned by this address
    balance: B256,
    // A 256 bit hash of the root node of the trie that encode the storage content of an account
    // (a mapping between 256 bit integer values)
    // encoded into the trie as: keccak256(all keys) -> RLP encoded of all values
    // Note: storage_root is not a "physical" member of the AccountState and does not contribute to it serialization
    storage_root: Bytes,
    // The hash of the EVM byte codes of this address
    code_hash: Bytes,

}