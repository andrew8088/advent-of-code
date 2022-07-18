use std::fs::File;
use std::io::Read;
use regex::Regex;

enum LightCommand {
    On { x1: usize, y1: usize, x2: usize, y2: usize } ,
    Off { x1: usize, y1: usize, x2: usize, y2: usize },
    Toggle { x1: usize, y1: usize, x2: usize, y2: usize },
    None
}

fn main() {
    let input = get_input();
    let parts = input.trim().split("\n");

    let mut map1 = get_map(1000, 1000);
    let mut map2 = get_map(1000, 1000);

    let part_1_to_true = |_x: usize| 1;
    let part_1_to_false = |_x: usize| 0;
    let part_1_toggle = |x: usize| if x == 1 { 0 } else { 1 };

    let part_2_to_true = |x: usize| x + 1;
    let part_2_to_false = |x: usize| if x == 0 { 0 } else { x - 1 };
    let part_2_toggle = |x: usize| x + 2;


    for part in parts {
        let cmd = parse_line(part);

        match cmd {
            LightCommand::On { x1, y1, x2, y2 } => {
                set_range(&mut map1, x1, y1, x2, y2, part_1_to_true);
                set_range(&mut map2, x1, y1, x2, y2, part_2_to_true);
            },
            LightCommand::Off { x1, y1, x2, y2 } => {
                set_range(&mut map1, x1, y1, x2, y2, part_1_to_false);
                set_range(&mut map2, x1, y1, x2, y2, part_2_to_false);
            },
            LightCommand::Toggle { x1, y1, x2, y2 } => {
                set_range(&mut map1, x1, y1, x2, y2, part_1_toggle);
                set_range(&mut map2, x1, y1, x2, y2, part_2_toggle);
            },
            LightCommand::None => {}
        }
    }

    let count1 = count_lights(map1);
    let count2 = count_lights(map2);
    println!("lit count (part 1): {}", count1);
    println!("lit count (part 2): {}", count2);
}

fn count_lights(map: Vec<Vec<usize>>) -> usize {
    let mut count = 0;

    for row in map {
        for cell in row {
            count += cell;
        }
    }
    count
}

fn set_range<F: FnMut(usize) -> usize>(map: &mut Vec<Vec<usize>>, x1: usize, y1: usize, x2: usize, y2: usize, mut f: F) {
    for x in x1..=x2 {
        let row = map.get_mut(x).unwrap();

        for y in y1..=y2 {
            if let Some(v) = row.get(y) {
                row[y] = f(*v);
            }
        }
    }
}

fn get_map(x_len: usize, y_len: usize) -> Vec<Vec<usize>> {
    let mut map = Vec::new();

    for _x in 0..x_len {
        let mut row: Vec<usize> = Vec::new();
        for _y in 0..y_len {
            row.push(0);
        }
        map.push(row);
    }
    map
}

fn parse_line (line: &str) -> LightCommand {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    match re.captures(line) {
        Some(caps) => {
            let command = caps.get(1).unwrap().as_str();

            let x1: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let y1: usize = caps.get(3).unwrap().as_str().parse().unwrap();
            let x2: usize = caps.get(4).unwrap().as_str().parse().unwrap();
            let y2: usize = caps.get(5).unwrap().as_str().parse().unwrap();

            match command {
                "turn on" => LightCommand::On { x1,  y1, x2, y2 },
                "turn off" => LightCommand::Off { x1,  y1, x2, y2 },
                "toggle" => LightCommand::Toggle { x1,  y1, x2, y2 },
                _ => LightCommand::None
            }
        },
        None => LightCommand::None
    }
}


// fn get_test_input() -> String {
//     return String::from("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500");
//     // return String::from("turn on 0,0 through 9,9\ntoggle 0,0 through 9,0\nturn off 4,4 through 5,5");
//     // return String::from("turn on 0,0 through 9,9\ntoggle 0,0 through 9,0");
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

