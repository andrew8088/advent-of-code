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
                    check_cycle(cycle, x, &mut strength_sum, &mut pixels);
                }
                l => {
                    let v: isize = l.split(' ').last().unwrap().parse().unwrap();
                    cycle += 1;
                    check_cycle(cycle, x, &mut strength_sum, &mut pixels);
                    cycle += 1;
                    check_cycle(cycle, x, &mut strength_sum, &mut pixels);
                    x += v;
                }
            }
        }

        println!("part 1: {}", strength_sum);
        println!("part 2: {:?}", pixels);
    });
}

fn check_cycle(cycle: isize, x: isize, sum: &mut isize, pixels: &mut Vec<char>) {
    if cycle >= 20 && (cycle - 20) % 40 == 0 {
        let signal_strength = cycle * x;
        println!("cycle {} - x={}, strength={}", cycle, x, signal_strength);

        *sum += signal_strength;
    }

    // let pixel_index = cycle - 1;
    // let sprite_pos = x - 1..x + 2;

    // println!("{} in {:?}?", pixel_index, sprite_pos);

    // pixels.push(if sprite_pos.contains(&pixel_index) {
    //     '#'
    // } else {
    //     '.'
    // })
}
