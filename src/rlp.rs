pub enum RLPInput<T> {
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

fn rl<T: 'static>(x: Vec<T>) -> Vec<u8> {
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
fn serialize<T: 'static>(x: Vec<T>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for item in x {
        result.extend(rlp(RLPInput::<u8>::Bytes(serialize_item(item))))
    }
    result
}

fn serialize_item<T: 'static>(item: T) -> Vec<u8> {
    match item {
        item if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u8>() => {
            vec![unsafe { std::mem::transmute_copy(&item) }]
        },
        item if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Vec<u8>>() => {
            unsafe { std::mem::transmute_copy::<T, Vec<u8>>(&item) }
        },
        _ => rlp(RLPInput::List(item)),
    }
}

// rlp(x) = rb(x) if x is bytes
// else rl(x)
pub fn rlp<T: 'static>(x: RLPInput<T>)-> Vec<u8> {
    match x {
        RLPInput::Bytes(bytes) => rb(bytes),
        RLPInput::List(list) => rl(list),
    }
}