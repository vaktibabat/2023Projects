use std::io;
use std::io::prelude::*;
use std::fs::File;

fn pop(mut x: u32) -> u32 {
    x = x - ((x >> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
    x = (x + (x >> 4)) & 0x0f0f0f0f;
    x = x + (x >> 8);
    x = x + (x >> 16);
    return x & 0x0000003f;
}

// Returns the bits of the integer in a vector, like this: 0xf -> [1, 1, 1, 1]
fn hex_to_binary(mut x: u8) -> Vec<u8> {
    let mut bef_pad = Vec::new();
    let mut cnt = 0;
    let mut res = Vec::new();

    while x > 0 {
        bef_pad.push(x % 2);

        x = x / 2;
       cnt += 1;
    }

    for _ in 0..(8 - bef_pad.len()) {
        res.push(0);
    }

    for x in bef_pad.iter().rev() {
        res.push(*x);
    }

    res
}

fn hamming_dist(s1: String, s2: String) -> u32{
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let mut s1_bits = Vec::new();
    let mut s2_bits = Vec::new();
    let mut xor_bits = Vec::new();
    let mut cnt = 0;

    // Compute the bits in each of the byte vectors
    for b in s1_bytes {
        let tmp = hex_to_binary(*b);

        for t in tmp {
            s1_bits.push(t);
        }
    }

    for b in s2_bytes {
        let tmp = hex_to_binary(*b);

        for t in tmp {
            s2_bits.push(t);
        }
    }

    let n = s1_bits.len();

    assert!(n == s2_bits.len());

    for i in 0..n {
        xor_bits.push(s1_bits[i] ^ s2_bits[i]);
    }

    for c in xor_bits {
        if c == 1 {
            cnt += 1;
        }
    }

    cnt 
}
fn score(s: Vec<u8>) -> u8 {
    let mut scr = 0;

    for c in s {
        if (0x30 <= c && c <= 0x09) || (0x41 <= c && c <= 0x5a) || (0x61 <= c && c <= 0x7a) || (c == 0x20) {
            scr = scr + 1;
        }
    }

    scr
}

fn find_top_ten(bytes: Vec<u8>) -> char {
    let n = bytes.len();
    let mut results = Vec::new();

    for curr_key in 1..=255 {
        let mut bytes_clone = Vec::new();
        let mut bytes_clone_clone = Vec::new();
        let mut encd = String::new();

        for x in 0..bytes.len() {
            bytes_clone.push(bytes[x]);
            bytes_clone_clone.push(bytes[x]);
        }

        for i in 0..n {
            bytes_clone[i] = bytes_clone[i] ^ curr_key;
            bytes_clone_clone[i] = bytes_clone[i];
        }
        
        let scr = score(bytes_clone_clone);

        for b in bytes_clone.iter() {
            encd.push(*b as char);
        }

        results.push((encd, scr, curr_key));
    }

    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    results.reverse();


    results[0].2.clone() as char
}        

fn main() -> io::Result<()> {
    let mut f = File::open("ciphertext.txt")?;
    let mut v = Vec::new();
    let n = f.read_to_end(&mut v)?;
    let mut res = Vec::new();
    let mut best_keysize = 0;
    let mut key = String::new();
    let mut trans_blocks = Vec::new();
    let mut plaintext = String::new();

    for _ in 0..29 {
        trans_blocks.push(Vec::new());
    }

    for curr_keysize in 2..41 {
        let first_block = String::from_utf8(v[0..curr_keysize].to_vec()).expect("Error");
        let second_block = String::from_utf8(v[curr_keysize..2 * curr_keysize].to_vec()).expect("Error");
        let third_block = String::from_utf8(v[2 * curr_keysize..3 * curr_keysize].to_vec()).expect("Error");
        let fourth_block = String::from_utf8(v[3 * curr_keysize..4 * curr_keysize].to_vec()).expect("Error");

        let dist = ((hamming_dist(first_block, second_block.clone()) +
                    hamming_dist(second_block, third_block.clone()) +
                    hamming_dist(third_block, fourth_block)) as f64) / ((curr_keysize * 3) as f64);
        
        res.push((curr_keysize, dist));
    }

    res.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    //After trying
    best_keysize = res[2].0;

    for i in 0..v.len() {
        trans_blocks[i % best_keysize].push(v[i]);
    }

    for b in trans_blocks {
        key.push(find_top_ten(b));
    }

    println!("{}", key);

    for i in 0..v.len() {
        plaintext.push((v[i] ^ (key.as_bytes())[i % best_keysize]) as char);
    }

    println!("plaintext: {}", plaintext);

    Ok(())
}
