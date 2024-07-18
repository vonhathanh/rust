use alloy_primitives::{Address, Bytes, B256, U256};

pub struct Transaction {
    // EIP-2718 transaction type; formally Tx
    tx_type: u8,
    // A scalar value equal to the number of transactions sent by the sender; formally Tn
    nonce: U256,
    // A scalar value equal to the maximum amount of gas that should be used in executing
    // this transaction. This is paid up-front,
    // before any computation is done and may not be increased later; formally Tg
    gas_limit: u64,
    // The 160-bit address of the message call’s recipient or, for a contract creation transaction, ∅
    // used here to denote the only member of B0 ; formally Tt
    to: Option<Address>,
    // A scalar value equal to the number of Wei to be transferred to the message call’s recipient or,
    // in the case of contract creation, as an endowment to the newly created account; formally Tv
    value: U256,
    // Values corresponding to the signature of the transaction and used to determine the sender of
    // the transaction; formally Tr and Ts
    r: B256,
    s: B256,
    // List of access entries to warm up; formally TA . Each access list entry E is a tuple
    // of an account address and a list of storage keys: E ≡ (Ea , Es )
    access_list: Option<Vec<(Address, Vec<B256>)>>,
    // Chain ID; formally Tc . Must be equal to the network chain ID β
    chain_id: Option<u64>,
    // Signature Y parity; formally Ty
    y_parity: Option<u8>,
    // Legacy transactions do not have an accessList (TA =()),
    // while chainId and yParity for legacy transactions are combined into a single value w
    w: Option<u64>,
    // A scalar value equal to the maximum number of Wei to be paid per unit of gas for
    // all computation costs incurred as a result of the execution of this transaction; formally Tm
    max_fee_per_gas: Option<U256>,
    // A scalar value equal to the maximum number of Wei to be paid to the
    // block’s fee recipient as an incentive to include the transaction; formally Tf .
    max_priority_fee_per_gas: Option<U256>,
    // A scalar value equal to the number of Wei to be paid per unit of gas for all computation
    // costs incurred as a result of the execution of this transaction; formally Tp
    gas_price: Option<U256>,
    // An unlimited size byte array specifying the EVM-code for the account initialisation procedure,
    // formally Ti
    init: Option<Bytes>,
    // An unlimited size byte array specifying the input data of the message call, formally Td
    data: Option<Bytes>,
}

impl Transaction {
    pub fn new(
        tx_type: u8,
        nonce: U256,
        gas_limit: u64,
        to: Option<Address>,
        value: U256,
        r: B256,
        s: B256,
        access_list: Option<Vec<(Address, Vec<B256>)>>,
        chain_id: Option<u64>,
        y_parity: Option<u8>,
        w: Option<u64>,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
        gas_price: Option<U256>,
        init: Option<Bytes>,
        data: Option<Bytes>,
    ) -> Self {
        Self {
            tx_type: tx_type,
            nonce: nonce,
            gas_limit: gas_limit,
            to: to,
            value: value,
            r: r,
            s: s,
            access_list: access_list,
            chain_id: chain_id,
            y_parity: y_parity,
            w: w,
            max_fee_per_gas: max_fee_per_gas,
            max_priority_fee_per_gas: max_priority_fee_per_gas,
            gas_price: gas_price,
            init: init,
            data: data,
        }
    }
}
