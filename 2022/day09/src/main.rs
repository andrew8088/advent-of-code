use std::collections::HashSet;

use sscanf::sscanf;
use util::get_lines;

fn main() {
    get_lines(|lines| {
        let motions = lines
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| sscanf!(l, "{} {}", char, usize).unwrap())
            .collect::<Vec<(char, usize)>>();

        println!("part 1: {}", solve(&motions, 2));
        println!("part 1: {}", solve(&motions, 10));
    })
}

fn solve(motions: &Vec<(char, usize)>, count: usize) -> usize {
    let mut knots = Vec::new();
    let mut knot_locs = Vec::new();

    for i in 0..count {
        knots.push((0, 0));
        knot_locs.push(HashSet::new());
        knot_locs[i].insert(knots[i].clone());
    }

    for (dir, count) in motions {
        for _ in 0..*count {
            match dir {
                'R' => knots[0].0 += 1,
                'L' => knots[0].0 -= 1,
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                _ => panic!("unknown direction {}", dir),
            }

            for idx in 0..knots.len() - 1 {
                let prev = knots[idx];
                let mut next = &mut knots[idx + 1];

                match (prev.0 - next.0, prev.1 - next.1) {
                    (2, 0) => {
                        next.0 += 1;
                    }
                    (0, 2) => {
                        next.1 += 1;
                    }
                    (-2, 0) => {
                        next.0 -= 1;
                    }
                    (0, -2) => {
                        next.1 -= 1;
                    }
                    (1, 2) | (2, 1) | (2, 2) => {
                        next.0 += 1;
                        next.1 += 1;
                    }
                    (-2, 1) | (-1, 2) | (-2, 2) => {
                        next.0 -= 1;
                        next.1 += 1;
                    }
                    (2, -1) | (1, -2) | (2, -2) => {
                        next.0 += 1;
                        next.1 -= 1;
                    }
                    (-2, -1) | (-1, -2) | (-2, -2) => {
                        next.0 -= 1;
                        next.1 -= 1;
                    }
                    _ => {}
                }
                knot_locs[idx + 1].insert(next.to_owned());
            }
        }
    }
    return knot_locs.last().unwrap().len();
}

// fn print_knots(knots: &Vec<(isize, isize)>) {
//     let mut graph: Vec<Vec<char>> = vec![vec!['.'; 30]; 30];

//     for (idx, (x, y)) in knots.iter().enumerate().rev() {
//         println!("{}: {}, {}", idx, x, y);
//         graph[((-1 * y) + 15) as usize][(x + 15) as usize] = if idx == 0 {
//             'H'
//         } else {
//             format!("{}", idx).chars().collect::<Vec<_>>()[0]
//         }
//     }

//     println!("--------------------------------------");
//     for row in graph {
//         for char in row {
//             print!("{}", char);
//         }
//         println!("");
//     }
//     println!("--------------------------------------");
// }
