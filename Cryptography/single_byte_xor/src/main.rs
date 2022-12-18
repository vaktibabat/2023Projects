use hex;

fn score(s: Vec<u8>) -> u8 {
    let mut scr = 0;

    for c in s {
        if (0x30 <= c && c <= 0x09) || (0x41 <= c && c <= 0x5a) || (0x61 <= c && c <= 0x7a) {
            scr = scr + 1;
        }
    }

    scr
}

fn find_top_ten(bytes: Vec<u8>) -> () {
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

        results.push((encd, scr));
    }

    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    results.reverse();

    println!("{:?} {:?}", results[0], results[1]);
}        

fn main() {
    let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let dec = hex::decode(s);

    match dec {
        Ok(v) => {
            find_top_ten(v);       
        },
        Err(e) => {
            println!("Error while decoding: {e:?}");
        }
    }
}
