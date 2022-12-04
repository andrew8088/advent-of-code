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
            func(content.trim().to_owned());
        }
    }
}

pub fn get_lines<F>(func: F)
where
    F: Fn(Vec<&str>),
{
    get_input(|content| {
        let lines: Vec<&str> = content.split('\n').map(|l| l.trim()).collect();
        func(lines)
    })
}
