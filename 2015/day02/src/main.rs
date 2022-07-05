use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::Read;

fn main() {
    let input = get_input();
    let lines = input.trim().split('\n');

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in lines {
        let dims = parse_line(line);
        let paper = calc_wrapping_paper(dims);
        let ribbon = calc_ribbon(dims);
        total_paper += paper;
        total_ribbon += ribbon;
    }

    println!("total paper: {}", total_paper);
    println!("total ribbon: {}", total_ribbon);
}

fn parse_line(line: &str) -> (i32, i32, i32) {
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();

    let mut w = String::from("");
    let mut h = String::from("");
    let mut d = String::from("");

    for cap in re.captures_iter(line) {
        w.push_str(&cap[1]);
        h.push_str(&cap[2]);
        d.push_str(&cap[3]);
    }

    return (w.parse::<i32>().unwrap(), h.parse::<i32>().unwrap(), d.parse::<i32>().unwrap());
}

fn calc_wrapping_paper((w, h, d): (i32, i32, i32)) -> i32 {
    let wh = w * h;
    let wd = w * d;
    let hd = h * d;

    let min = cmp::min(wh, cmp::min(wd, hd));

    return (2 * wh) + (2 * wd) + (2 * hd) + min;
}

fn calc_ribbon((w, h, d): (i32, i32, i32)) -> i32 {
    let perim1 = (2*w) + (2*h);
    let perim2 = (2*w) + (2*d);
    let perim3 = (2*h) + (2*d);
    let min_perim = cmp::min(perim1, cmp::min(perim2, perim3));

    return min_perim + (w*h*d);
}

// fn get_test_input() -> String {
//     return String::from("2x3x4\n1x1x10");
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
