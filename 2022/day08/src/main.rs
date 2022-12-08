use take_until::TakeUntilExt;
use util::{get_lines, transpose};

fn main() {
    get_lines(|lines| {
        let row_trees: Vec<_> = lines
            .iter()
            .filter(|l| !l.trim().is_empty())
            .map(|line| line.chars().map(parse_char).collect::<Vec<usize>>())
            .collect();

        let col_count = row_trees[0].len();
        let row_count = row_trees.len();

        let col_trees = transpose(&row_trees);

        let mut visible_count = 0;
        let mut max_scenic_score = 0;

        for x in 0..row_count {
            for y in 0..col_count {
                let tree = row_trees[x][y];
                let top = &col_trees[y][0..x];
                let bottom = &col_trees[y][(x + 1)..];
                let left = &row_trees[x][0..y];
                let right = &row_trees[x][(y + 1)..];
                if is_visible(tree, top, bottom, left, right) {
                    visible_count += 1;
                }
                let scenic_score = get_scenic_score(tree, top, bottom, left, right);

                if scenic_score > max_scenic_score {
                    max_scenic_score = scenic_score;
                }
            }
        }

        println!("part 1: {}", visible_count);
        println!("part 2: {}", max_scenic_score);
    });
}

fn parse_char(c: char) -> usize {
    c as usize - '0' as usize
}

fn is_visible(
    value: usize,
    top: &[usize],
    bottom: &[usize],
    left: &[usize],
    right: &[usize],
) -> bool {
    for side in [top, bottom, left, right] {
        match side.iter().max() {
            Some(max) => {
                if *max < value {
                    return true;
                }
            }
            None => {
                return true;
            }
        }
    }

    return false;
}

fn get_scenic_score(
    value: usize,
    top: &[usize],
    bottom: &[usize],
    left: &[usize],
    right: &[usize],
) -> usize {
    let higher_or_equal = |v: &&usize| **v >= value;

    let mut score = 1;

    for (idx, side) in [top, bottom, left, right].iter().enumerate() {
        let should_rev = idx % 2 == 0;

        let iter = side.iter();

        let view = if should_rev {
            iter.rev().take_until(higher_or_equal).collect::<Vec<_>>()
        } else {
            iter.take_until(higher_or_equal).collect::<Vec<_>>()
        };

        score *= view.len();
    }

    score
}
