use regex::Regex;
use md5::{Md5, Digest};

fn main() {
    let input = "iwrupvqb";
    let mut counter = 0;

    loop {
        let val = format!("{}{}", input, counter);
        let hash = Md5::digest(val.as_str());
        let hash_hex = format!("{:x}", hash);
        let has_5_zeros = has_leading_zeros(hash_hex.as_str(), 5);
        let has_6_zeros = has_leading_zeros(hash_hex.as_str(), 6);
        counter += 1;

        if has_5_zeros {
            println!("5 zeros: {} => {}", val, hash_hex);
        }
        if has_6_zeros {
            println!("6 zeros: {} => {}", val, hash_hex);
            break;
        }
    }
}

fn has_leading_zeros(hash: &str, zeros: u32) -> bool {
    let raw_re = format!("^0{{{}}}", zeros);
    let re = Regex::new(raw_re.as_str()).unwrap();
    return re.is_match(hash);
}
