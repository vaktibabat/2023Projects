use base64;
use hex;

fn main() {
    let to_encode = "deadbeef";
    let hex_decoded = hex::decode(to_encode);

    match hex_decoded {
        Ok(v) => {
            let encoded = base64::encode(v);

            println!("Succeeded: {}", encoded);
        },
        Err(e) => {
            println!("Error: {e:?}");
        },
    }
}
