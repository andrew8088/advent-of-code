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

        dirs.push(String::from("/"));

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

        let mut sum = 0;
        let mut dir_sizes: HashMap<String, usize> = HashMap::new();

        for dir in dirs {
            let mut dir_size = 0;
            for (path, size) in &files {
                if path.contains(&dir) {
                    dir_size += size;
                }
            }

            dir_sizes.insert(dir, dir_size);

            if dir_size < 100_000 {
                sum += dir_size;
            }
        }

        println!("part 1: {}", sum);

        let unused_space = TOTAL_DISK_SPACE - dir_sizes.get("/").unwrap();

        let mut min_dir_to_delete = REQUIRED_DISK_SPACE;

        for (_, size) in &dir_sizes {
            let empty_disk_space = size + unused_space;
            if empty_disk_space > REQUIRED_DISK_SPACE && *size < min_dir_to_delete {
                min_dir_to_delete = *size;
            }
        }

        println!("part 2: {}", min_dir_to_delete);
    });
}


