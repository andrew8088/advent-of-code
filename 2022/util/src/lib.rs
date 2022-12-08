use std::env;
use std::fs;

fn get_file_name() -> &'static str {
    if env::var("DEMO").unwrap_or("0".to_string()) == "1" {
        "demo-input.txt"
    } else {
        "input.txt"
    }
}

pub fn get_input<F>(func: F)
where
    F: Fn(String),
{
    let file_name = get_file_name();
    println!("loading {}", file_name);
    match fs::read_to_string(file_name) {
        Err(_) => println!("Cannot load file"),
        Ok(content) => {
            func(content.to_owned());
        }
    }
}

pub fn get_lines<F>(func: F)
where
    F: Fn(Vec<&str>),
{
    get_input(|content| {
        let lines: Vec<&str> = content.split('\n').collect();
        func(lines)
    })
}

pub fn transpose<T>(rows: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..rows[0].len())
        .map(|i| {
            rows.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<T>>>()
}
