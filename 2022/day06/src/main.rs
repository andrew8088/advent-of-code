use std::{
    collections::{HashSet, VecDeque},
    str::Chars,
};
use util::get_input;

fn main() {
    get_input(|content| {
        let chars1 = content.trim().chars();
        let chars2 = chars1.clone();

        println!("part 1: after {} chars", find_index(chars1, 4).unwrap());
        println!("part 1: after {} chars", find_index(chars2, 14).unwrap());
    });
}

fn find_index(content: Chars, uniq_count: usize) -> Option<usize> {
    let mut window: VecDeque<char> = VecDeque::new();
    let mut set: HashSet<char> = HashSet::new();
    for (idx, c) in content.enumerate() {
        window.push_back(c);
        set.insert(c);

        if window.len() < uniq_count {
            continue;
        }

        if window.len() > uniq_count {
            let old_char = window.pop_front().unwrap();
            if !window.contains(&old_char) {
                set.remove(&old_char);
            }
        }

        if window.len() == set.len() {
            return Some(idx + 1);
        }
    }

    return None;
}
