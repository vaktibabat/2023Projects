use hex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn score(s: Vec<u8>) -> u8 {
    let mut scr = 0;

    for c in s {
        if (0x30 <= c && c <= 0x09) || (0x41 <= c && c <= 0x5a) || (0x61 <= c && c <= 0x7a) || (c == 0x20) {
            scr = scr + 1;
        }
    }

    scr
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_top_ten(bytes: Vec<u8>) -> (String, u8) {
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

    results[0].clone()
}        

fn main() {
    let mut results = Vec::new();

    if let Ok(lines) = read_lines("./enc_file") {
        for line_err in lines {
            if let Ok(line) = line_err {
                if let Ok(dec) = hex::decode(line) {
                    let first = find_top_ten(dec);
                    let first_len = (first.0).len();
                    let curr_score = (first.1 as f64) / (first_len as f64);

                    results.push((first.0, curr_score));
                }
            }
        }
    }

    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    results.reverse();

    println!("{:?}", results[0]);
}
