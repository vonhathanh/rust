use rust_tutorial::rlp::*;

#[test]
fn polynomial() {
    // let rb_result = rlp(RLPInput::<u8>::Bytes(vec![1, 2, 3, 4]));
    // println!("rb: {:?}", rb_result);
    rlp(RLPInput::List(vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]));
}
