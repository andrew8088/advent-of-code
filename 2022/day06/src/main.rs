use std::collections::HashSet;
use util::get_input;

fn main() {
    get_input(|content| {
        println!("part 1: after {} chars", find_index(&content, 4).unwrap());
        println!("part 2: after {} chars", find_index(&content, 14).unwrap());
    });
}

fn find_index(content: &str, uniq_count: usize) -> Option<usize> {
    for e in uniq_count..content.len() + 1 {
        let s: HashSet<char> = content[e - uniq_count..e].chars().into_iter().collect();

        if s.len() == uniq_count {
            return Some(e);
        }
    }

    None
}
