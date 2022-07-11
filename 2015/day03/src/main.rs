use std::collections::HashSet;
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
    houses: HashSet<(i32, i32)>,
    x: i32,
    y: i32,
}

fn get_new_santa () -> Santa {
    return Santa {
        houses: HashSet::from([(0, 0)]),
        x: 0,
        y: 0,
    };
}

fn part1(parts: Split<&str>) {
    let mut santa = get_new_santa();

    for part in parts {
        apply_part(part, &mut santa);
    }

    println!("total house count: {}", santa.houses.len());
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

    let all_houses: HashSet<_> = real_santa.houses.union(&robo_santa.houses).collect();
    let set_total = all_houses.len();

    println!("unique house count: {}", set_total);
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

    santa.houses.insert((santa.x, santa.y));
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
