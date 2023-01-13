use std::fs;

fn main() {
    match fs::read_to_string("input.txt") {
        Err(_) => println!("Cannot load file"),
        Ok(content) => {
            let counts = content.split("\n\n").map(|rows| -> usize {
                rows.split('\n').map(|row| row.parse().unwrap_or(0)).sum()
            });

            let mut v: Vec<_> = counts.collect();

            v.sort();

            let last_idx = v.len() - 1;
            println!("the highest count is {}", v[last_idx]);

            // let len = v.len();

            // let top_three: usize = v.split_off(len - 3).into_iter().sum();
            // println!("sum of top 3 calorie counts: {:?}", top_three);
        }
    }
}
