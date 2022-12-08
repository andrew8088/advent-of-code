use std::collections::VecDeque;
use spiral::ChebyshevIterator;

use util::get_lines;

fn main() {
    get_lines(|lines| {
        let col_count = lines[0].len();
        let row_count = lines.len();
        let cells: VecDeque<_> = lines.iter().map(|line| line.chars().map(parse_char).collect::<VecDeque<usize>>()).collect();

        // let maxes: Vec<Vec<usize>> = Vec::new();

        // - r1f
        // - r1b
        
        let center_x = col_count / 2;
        let center_y = row_count / 2;
        let coords = ChebyshevIterator::new(center_x, center_y, center_x + 1).collect::<Vec<(usize, usize)>>();
        let iterator = coords.iter().rev();

        for (x, y) in iterator {
            println!("{} {} - {}", x, y, cells[*x][*y]);




        }

        // for i in 0..col_count {
        //     for j in 0..row_count {
        //         println!("{} {} - {}", i, j, cells[i][j]);
        //     }
        // }

    });
}

fn parse_char (c: char) -> usize {
    c as usize - '0' as usize
}

