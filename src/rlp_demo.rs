pub enum RLPInput<T: IntoIterator> {
    Bytes(Vec<u8>),
    List(Vec<T>)
}

// if x is a single byte and x < 128, return x
// else if x.len < 56, return (128+||x||).concat(x)
// else return (183 + ||BE(||x||)||).concat(BE(||x||)).concat(x)
// else return empty 
fn rb(x: Vec<u8>) -> Vec<u8> {
    println!("RB called");

    let mut result: Vec<u8> = vec![];

    if x.len() == 1 && x[0] < 128 {
        result.push(x[0]);

    } else if x.len() < 56 {
        result.push(128 + x.len() as u8);
        result.extend(x);

    } else if x.len() < usize::MAX {
        let bex = x.len().to_be_bytes().to_vec();

        result.push(183 + bex.len() as u8);
        result.extend(bex);
        result.extend(x);
    }

    result
}
// [item 1, item 2, item 3]
// item 1 = vec<u8>, item 2 = [item 2.1, item 2.2]
fn rl<T: IntoIterator>(x: Vec<T>) -> Vec<u8> {
    println!("RL called");

    let mut result: Vec<u8> = vec![];

    let sx = serialize(x);
    // If the concatenated serialisations (sx) of each contained item is less than 56 bytes in length, 
    // return (192 + ||sx||).concat(sx)
    if sx.len() < 56 && sx.len() > 0 {
        result.push(192 + sx.len() as u8);
        result.extend(sx);
    } 
    // Otherwise, if sx < 2^64 and sx != empty list
    // return (247 + ||BE(||sx||)||).concat(BE(||sx||)).concat(sx)
    else if sx.len() < u64::MAX.try_into().unwrap() {
        let besx = sx.len().to_be_bytes().to_vec();

        result.push(247 + besx.len() as u8);
        result.extend(besx);
        result.extend(sx);
    }

    result
}

// run rlp() for every item in x
fn serialize<T: IntoIterator>(x: Vec<T>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    // for item in x {
    //     result.extend(rlp(RLPInput::<Vec<u8>>::Bytes(serialize_item(item))))
    // }
    result
}

// rlp(x) = rb(x) if x is bytes
// else rl(x)
pub fn rlp<T: IntoIterator>(x: RLPInput<T>)-> Vec<u8> {
    match x {
        RLPInput::Bytes(bytes) => rb(bytes),
        RLPInput::List(list) => rl(list),
    }
}