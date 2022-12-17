use util::{get_grid, print_grid, CharGrid};

type Point = (usize, usize);

fn main() {
    get_grid(|grid| {
        print_grid(&grid);

        let mut path = Vec::new();

        path.push((0, 0));

        next_step(&grid, &(0, 0));
    });
}

fn next_step(grid: &CharGrid, curr: &Point) {
    println!("curr: {:?}", curr);

    for n in neighbors(curr, grid) {
        println!("- {:?}", n);
    }
}

fn neighbors(p: &Point, grid: &CharGrid) -> Vec<Point> {
    let (x, y) = p;
    let row_count = grid.len();
    let col_count = grid[*x].len();

    let mut neighbors = Vec::new();

    if *x != 0 {
        neighbors.push((x - 1, *y));
    }

    if *x != row_count - 1 {
        neighbors.push((x + 1, *y));
    }

    if *y != 0 {
        neighbors.push((*x, y - 1));
    }

    if *y != col_count - 1 {
        neighbors.push((*x, y + 1));
    }

    neighbors
}
