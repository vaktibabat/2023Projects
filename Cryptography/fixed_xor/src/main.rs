use hex;

fn fixed_xor(b1: Vec<u8>, b2: Vec<u8>) -> String {
    let mut ret = Vec::new();

    assert!(b1.len() == b2.len());

    let n = b1.len();

    for i in 0..n {
        ret.push(b1[i] ^ b2[i]);
    }

    hex::encode(ret)
}

fn main() {
    let s1 = "deadbeef";
    let s2 = "feebdaed";
    let s1_bytes = hex::decode(s1);
    let s2_bytes = hex::decode(s2);


    match s1_bytes {
        Ok(v_1) => {
            match s2_bytes {
                Ok(v_2) => {
                    println!("{:?}", fixed_xor(v_1, v_2));
                }
                
                Err(e_2) => {
                    println!("Error while decoding the second string.");
                }
            }
        }
        Err(e_1) => {
            println!("Error while decoding the first string.");
        }
    }
}
