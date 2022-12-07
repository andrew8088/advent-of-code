use std::collections::HashMap;
use util::get_lines;
use sscanf::sscanf;

const TOTAL_DISK_SPACE: usize = 70_000_000;
const REQUIRED_DISK_SPACE: usize = 30_000_000;

fn main() {
    get_lines(|lines| {
        let mut pwd = Vec::new();
        let mut dir_sizes: HashMap<String, usize> = HashMap::new();

        for line in lines { 
            if let Ok(dir) = sscanf!(line, "$ cd {}", str) {
                match dir {
                    ".." => { pwd.pop(); },
                    _ => pwd.push(dir),
                }
            } else if let Ok((size, _)) = sscanf!(line, "{} {}", usize, str) {
                for i in 0..pwd.len() {
                    let path = pwd[0..i+1].join("/");
                    let curr_size = dir_sizes.get(&path).unwrap_or(&0);
                    dir_sizes.insert(path, curr_size + size);
                }
            }
        }

        let mut part1_sum = 0;
        let mut part2_min = REQUIRED_DISK_SPACE;

        let unused_space: usize = TOTAL_DISK_SPACE - dir_sizes.get("/").unwrap_or(&0);

        for (_, size) in dir_sizes {
            if size < 100_000 {
                part1_sum += size;
            }

            if (size + unused_space) > REQUIRED_DISK_SPACE && size < part2_min {
                part2_min = size;
            }
        }

        println!("part 1: {}", part1_sum);
        println!("part 2: {}", part2_min);
    });
}
