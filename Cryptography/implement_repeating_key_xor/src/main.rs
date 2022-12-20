use hex;

fn repeating_key_xor(key: Vec<u8>, mut to_encrypt: Vec<u8>) -> Vec<u8> {
    let n = to_encrypt.len();
    let k = key.len();

    let mut v_clone = Vec::new();

    for i in 0..n {
        v_clone.push(to_encrypt[i] ^ key[i % k]);
    }

    v_clone
}

fn main() {
    let s_str = "414141414141";
    let key_str = "ICE";

    let s_bytes = hex::decode(s_str);
    let key_bytes = key_str.as_bytes();

    match s_bytes {
        Ok(v) => {
            let res = repeating_key_xor(key_bytes.to_vec(), v);

            println!("{:?}", hex::encode(res));
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
