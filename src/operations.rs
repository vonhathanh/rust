enum Opcode {
    STOP,
    ADD,
    MUL,    
    SUB,
    DIV,
    LT,
    GT,
    SLT,
    EQ,
    ADDRESS,
    BALANCE,
    ORIGIN,
    POP,
    MLOAD,
    MSTORE,
    SLOAD,
    SSTORE,
    JUMP,
    PUSH1,
    PUSH32,
    DUP1,
    DUP32,
    LOG1,
    LOG4,
    CREATE,
    CALL,
    SELFDESTRUCT
}

pub struct Operation {
    pub index: u8,
    pub name: Opcode,
    pub delta: u8,
    pub alpha: u8,
    pub gas: u8,
}

impl Operation {
    pub const fn new(index: u8, name: Opcode, delta: u8, alpha: u8, gas: u8) -> Self {
        Operation { index, name, delta, alpha, gas }
    }
}

const ADD_OP: Operation = Operation::new(0x01, Opcode::ADD, 2, 1, 10);
const OPERATIONS: [Operation; 1] = [ADD_OP];
