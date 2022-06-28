use std::fs::File;
use std::io::Read;

fn main() {
    let mut floor = 0;
    let mut current_index = 0;
    let mut basement_index = 0;

    for char in get_input().split("") {
        if char == "(" {
            floor += 1;
        } else if char == ")" {
            floor -= 1;
        } else {
            continue;
        }
        current_index += 1;

        println!("{}, {},  {}", char, current_index, floor);

        if floor == -1 && basement_index == 0 {
            basement_index = current_index;
        }
    }

    println!("final floor: {}", floor);
    println!("basement index: {}", basement_index);
}

// fn get_test_input() -> String {
//     return String::from(")");
//     return String::from("()())");
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
