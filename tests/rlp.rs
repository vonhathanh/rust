use rust_tutorial::rlp::*;

#[test]
fn polynomial() {
    let bytes_input = Bytes::new(vec![1, 2, 3, 4]);
    rlp(bytes_input);
    let string_input = List::new(vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]);
    rlp(string_input);
}
