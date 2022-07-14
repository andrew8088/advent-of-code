use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use regex::Regex;

fn main() {
    let input = get_input();
    let parts = input.trim().split("\n");

    let mut nice_count_part1 = 0;
    let mut nice_count_part2 = 0;

    for line in parts {
        if is_nice_part1(line) {
            nice_count_part1 += 1;
        }

        if is_nice_part2(line) {
            nice_count_part2 += 1;
        }
    }
    println!("nice count (part1): {}", nice_count_part1);
    println!("nice count (part2): {}", nice_count_part2);
}

fn is_nice_part1(line: &str) -> bool {
    let naughty_strings = Regex::new("ab|cd|pq|xy").unwrap();

    if naughty_strings.is_match(line) {
        return false;
    }

    let vowels = Regex::new("[aeiou].*[aeiou].*[aeiou]").unwrap();

    if !vowels.is_match(line) {
        return false;
    }

    let double_letters_raw = ('b' ..= 'z').fold(String::from("aa"), |acc, next| format!("{}|{}{}", acc, next, next));
    let double_letter = Regex::new(&double_letters_raw).unwrap();

    if !double_letter.is_match(line) {
        return false;
    }

    true
}

fn is_nice_part2(line: &str) -> bool {
    let pairs = to_char_chunks(line, 2);
    let chunks = to_char_chunks(line, 3);

    has_repeated_pair(&pairs, &chunks) && has_xyx_pattern(&chunks)
}

fn has_repeated_pair(tuples: &Vec<Vec<char>>, thruples: &Vec<Vec<char>>) -> bool {
    let map = count_uniq(&tuples);
    let repeated_pairs = map.iter().filter(|(_, v)| **v > 1).collect::<Vec<(&&Vec<char>, &u32)>>();
    let overlapping_tuples: Vec<&Vec<char>>= thruples.iter().filter(|v| v.get(0) == v.get(1) && v.get(1) == v.get(2)).collect();

    repeated_pairs.len() > 0 && overlapping_tuples.len() == 0
}

fn has_xyx_pattern(chunks: &Vec<Vec<char>>) -> bool {
    for chunk in chunks {
        if chunk.get(0) == chunk.get(2) {
            return true;
        }
    }
    false
}

fn count_uniq<T : Eq + Hash>(set: &Vec<T>) -> HashMap<&T, u32> {
    let mut map = HashMap::new();

    for key in set {
        let val = match map.get(&key) {
            Some(curr) => curr + 1,
            None => 1
        };

        map.insert(key, val);
    }

    map
}

fn to_char_chunks(line: &str, chunk_len: usize) -> Vec<Vec<char>> {
    let mut chunks: Vec<Vec<char>> = Vec::new();

    let mut curr_vec: Vec<char> = Vec::new();

    for c in line.chars() {
        curr_vec.push(c);

        if curr_vec.len() == chunk_len {
            chunks.push(curr_vec.clone());
            curr_vec.remove(0);
        }
    }
    chunks
}


// fn get_test_input() -> String {
//     //return String::from("ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb\nqjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy");
//     return String::from("aaa\nqjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy");
// }

fn get_input() -> String {
    let filename = "./input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            return content;
        },
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
            return String::from("");
        },
    }
}

