use alloy_primitives::{Address, Bytes, B256, U256};

#[derive(Default)]
pub struct BlockHeader {
    // parentHash: The Keccak 256-bit hash of the parent block’s header, in its entirety; formally Hp.
    parent_hash: B256,
    // ommersHash: A 256-bit hash field that is now deprecated due to the replacement of proof of work consensus. 
    // It is now to a constant, KEC(RLP(())); formally Ho.
    ommers_hash: B256,
    // beneficiary: The 160-bit address to which priority fees from this block are transferred; formally Hc.
    beneficiary: Address,
    // stateRoot: The Keccak 256-bit hash of the root node of the state trie, after all transactions are
    // executed and finalisations applied; formally Hr.
    state_root: B256,
    // transactionsRoot: The Keccak 256-bit hash of the root node of the trie structure populated with each
    // transaction in the transactions list portion of the block; formally Ht.
    transactions_root: B256,
    // receiptsRoot: The Keccak 256-bit hash of the root node of the trie structure populated with the receipts of 
    // each transaction in the transactions list portion of the block; formally He.
    receipts_root: B256,
    // logsBloom: The Bloom filter composed from indexable information (logger address and log topics)
    // contained in each log entry from the receipt of each transaction in the transactions list; formally Hb
    logs_bloom: Vec<B256>,
    // difficulty: A scalar field that is now deprecated due to the replacement of proof of work consensus. 
    // It is set to 0; formally Hd.
    difficulty: U256,
    // number: A scalar value equal to the number of ancestor blocks. The genesis block has a number of zero; formally Hi.
    number: U256,
    // gasLimit: A scalar value equal to the current limit of gas expenditure per block; formally Hl.
    gas_limit: u64,
    // gasUsed: A scalar value equal to the total gas used in transactions in this block; formally Hg.
    gas_used: u64,
    // timestamp: A scalar value equal to the reasonable output of Unix’s time() at this block’s inception; formally Hs.
    timestamp: u64,
    // extraData: An arbitrary byte array containing data relevant to this block. This must be 32 bytes or fewer; formally Hx.
    extra_data: Bytes,
    // prevRandao: the latest RANDAO mix of the post beacon state of the previous block; formally Ha.
    prev_randao: U256,
    // nonce: A 64-bit value that is now deprecated due to the replacement of proof of work consensus. It
    // is set to 0x0000000000000000; formally Hn.
    nonce: u64,
    // baseFeePerGas: A scalar value equal to the amount of wei that is burned for each unit of gas consumed; formally Hf.
    base_fee_per_gas: U256,
}

impl BlockHeader {
    pub fn new() -> Self {
        BlockHeader::default()
    }
}