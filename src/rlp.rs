use std::fmt::Debug;

#[derive(Debug)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new(x: Vec<u8>) -> Self {
        Bytes(x)
    }
}

#[derive(Debug)]
pub struct List<T>(Vec<T>);

impl<T> List<T> {
    pub fn new(x: Vec<T>) -> Self {
        List(x)
    }
}

// Define an enum to distinguish between the types
#[derive(Debug)]
pub enum RLPInputType {
    BytesArray,
    ListOfItems
}

// define a trait that both types of input will implement
pub trait RLPInput: Debug {
    fn get_type(&self) -> RLPInputType;
}

// implement it for bytes array first
impl RLPInput for Bytes {
    fn get_type(&self) -> RLPInputType {
        RLPInputType::BytesArray
    }
}

// implement the trait for a list of arbitrary values
impl<T: Debug> RLPInput for List<T> {
    fn get_type(&self) -> RLPInputType {
        RLPInputType::ListOfItems
    }
} 

// rlp(x) = rb(x) if x is bytes
// else rl(x)


pub fn rlp<T: RLPInput>(x: T)-> Vec<u8> {
    match x.get_type() {
        RLPInputType::BytesArray => println!("Bytes array type, calling RB()"),
        RLPInputType::ListOfItems => println!("List of arbitrary values, call RL()"),
    }
    vec![]
}