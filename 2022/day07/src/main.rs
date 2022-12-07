use std::collections::HashMap;

use util::get_lines;
use sscanf::sscanf;


const TOTAL_DISK_SPACE: usize = 70_000_000;
const REQUIRED_DISK_SPACE: usize = 30_000_000;

fn main() {
    get_lines(|lines| {
        let mut pwd = Vec::new();
        let mut dirs = Vec::new();
        let mut files: HashMap<String, usize> = HashMap::new();

        for line in lines { 
            let clean_line = line.trim();

            if clean_line.is_empty() {
                continue;
            }

            let parsed: Result<&str, _> = sscanf!(clean_line, "$ cd {}", str);

            match parsed {
                Ok(dir) => {
                    match dir {
                        "/" => pwd.push(""),
                        ".." => { pwd.pop(); },
                        _ => pwd.push(dir),
                    }
                },
                Err(_) => {
                    if clean_line == "$ ls" {
                        continue;
                    }

                    match sscanf!(clean_line, "{} {}", usize, str) {
                        Ok((size, filename)) => {
                            files.insert(format!("{}/{}", pwd.join("/"), filename), size);
                        },
                        Err(_) => {
                            match sscanf!(clean_line, "dir {}", str) {
                                Ok(dir) => dirs.push(format!("{}/{}", pwd.join("/"), dir)),
                                Err(_) => panic!("unreachable: {}", clean_line),
                            }
                        },
                    }
                }
            }
        }

        let mut part1_sum = 0;
        let mut part2_min = REQUIRED_DISK_SPACE;

        let unused_space: usize = TOTAL_DISK_SPACE - files.values().sum::<usize>();

        for dir in dirs {
            let mut dir_size = 0;
            for (path, size) in &files {
                if path.contains(&dir) {
                    dir_size += size;
                }
            }

            if dir_size < 100_000 {
                part1_sum += dir_size;
            }

            let empty_disk_space = dir_size + unused_space;
            if empty_disk_space > REQUIRED_DISK_SPACE && dir_size < part2_min {
                part2_min = dir_size;
            }
        }

        println!("part 1: {}", part1_sum);
        println!("part 2: {}", part2_min);
    });
}
