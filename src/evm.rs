use crate::state::WorldState;

pub struct EVM {
    // Global system state
    worldState: WorldState,
    // Sub state
    subState: SubState,
    // A simple stack with word size = 256 bit. Maximum size of the stack is 1024
    stack: Stack,
    // A simple word-addressed byte array
    memory: Memory,
    // Unlike memory, which is volatile, storage is non volatile and is maintained as part of the system state
    storage: Storage,
    // Remaining gas for the execution
    gas: Gas,
    executionContext: Context,
}