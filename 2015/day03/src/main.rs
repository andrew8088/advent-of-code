use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::str::Split;

fn main() {
    let input = get_input();
    let parts = input.trim().split("");

    part1(parts.clone());
    part2(parts);
}

struct Santa {
    gifts: HashMap<i32, HashMap<i32, i32>>,
    x: i32,
    y: i32,
}

fn get_new_santa () -> Santa {
    let mut gifts: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut nested_map: HashMap<i32, i32> = HashMap::new();
    nested_map.insert(0, 1);
    gifts.insert(0, nested_map);

    return Santa {
        gifts,
        x: 0,
        y: 0,
    };
}

fn part1(parts: Split<&str>) {
    let mut santa = get_new_santa();

    for part in parts {
        apply_part(part, &mut santa);
    }

    println!("total house count: {}", count_houses(santa));
}

fn part2(parts: Split<&str>) {
    let mut real_santa = get_new_santa();
    let mut robo_santa = get_new_santa();

    let mut index = 0;

    for part in parts {
        index += 1;
        let santa = if index % 2 == 0 { &mut real_santa } else { &mut robo_santa };
        apply_part(part, santa);
    }

    let total_house_count = count_unique_houses(real_santa, robo_santa);

    println!("unique house count: {}", total_house_count);
}

fn apply_part(part: &str, santa: &mut Santa) {
    match part {
        "^" => { santa.y += 1 }
        ">" => { santa.x += 1 }
        "v" => { santa.y -= 1 }
        "<" => { santa.x -= 1 }
        _ => {
            return;
        }
    }

    match santa.gifts.get_mut(&santa.x) {
        Some(hm) => {
            match hm.get_mut(&santa.y) {
                Some(&mut val) => {
                    hm.insert(santa.y, val + 1);
                }
                None => {
                    hm.insert(santa.y, 1);
                }
            }
        },
        None => {
            let mut nested_map: HashMap<i32, i32> = HashMap::new();
            nested_map.insert(santa.y, 1);
            santa.gifts.insert(santa.x, nested_map);
        }
    };
}

fn count_houses(santa: Santa) -> i32 {
    let mut total = 0;

    for (_, nested_map) in santa.gifts {
        for _ in nested_map {
            total += 1;
        }
    }

    return total;
}

fn count_unique_houses(santa1: Santa, santa2: Santa) -> usize {
    let mut houses: HashSet<(i32, i32)> = HashSet::new();

    for (x, nested_map) in santa1.gifts {
        for (y, _) in nested_map {
            houses.insert((x, y));
        }
    }

    for (x, nested_map) in santa2.gifts {
        for (y, _) in nested_map {
            houses.insert((x, y));
        }
    }

    return houses.len();

}

// fn get_test_input() -> String {
//     // return String::from("^>v<");
//     return String::from("^v^v^v^v^v");
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
