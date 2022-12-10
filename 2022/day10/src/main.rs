use util::get_lines;

fn main() {
    get_lines(|lines| {
        let mut cycle = 0;
        let mut x = 1;
        let mut strength_sum: isize = 0;
        let mut pixels: Vec<char> = Vec::new();

        for line in lines {
            match line.trim() {
                "" => {}
                "noop" => {
                    cycle += 1;
                    run_cycle(cycle, x, &mut strength_sum, &mut pixels);
                }
                l => {
                    let v: isize = l.split(' ').last().unwrap().parse().unwrap();
                    cycle += 1;
                    run_cycle(cycle, x, &mut strength_sum, &mut pixels);
                    cycle += 1;
                    run_cycle(cycle, x, &mut strength_sum, &mut pixels);
                    x += v;
                }
            }
        }

        println!("part 1: {}", strength_sum);
        println!("part 2:");
        print_pixels(pixels);
    });
}

fn run_cycle(cycle: isize, x: isize, sum: &mut isize, pixels: &mut Vec<char>) {
    if cycle >= 20 && (cycle - 20) % 40 == 0 {
        *sum += cycle * x;
    }

    let pixel_index = (cycle - 1) % 40;

    pixels.push(if (x - 1..x + 2).contains(&pixel_index) {
        '#'
    } else {
        '.'
    });
}

fn print_pixels(pixels: Vec<char>) {
    for chunk in pixels.chunks(40) {
        println!("{}", chunk.iter().collect::<String>());
    }
}
