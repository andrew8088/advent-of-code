use std::fs;

fn main() {
    match fs::read_to_string("input.txt") {
        Err(_) => println!("Cannot load file"),
        Ok(content) => {
            let counts = content.split("\n\n").map(|calorie_rows| -> usize {
                calorie_rows.split('\n').map(|l| l.parse::<usize>().unwrap_or(0)).sum()
            });

            let mut v: Vec<usize> = counts.collect();

            let len = v.len();

            v.sort();

            let top_three: usize = v.split_off(len - 3).into_iter().sum();
            println!("sum of top 3 calorie counts: {:?}", top_three);
        }
    }
}
