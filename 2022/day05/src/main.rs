use util::get_lines;

fn main() {
    get_lines(|lines| {
        let mut iter = lines.iter();
        let crates = iter
            .by_ref()
            .cloned()
            .take_while(|l| !l.trim().is_empty()) // empty line will be consumed here
            .collect::<Vec<_>>();

        let commands = iter.cloned()
            .take_while(|l| !l.trim().is_empty()) // empty line will be consumed here
            .collect::<Vec<_>>();

        let mut stacks1 = parse_crates(crates);
        let mut stacks2 = stacks1.clone();
        let commands = parse_commands(commands);

        perform_part1(&mut stacks1, &commands);
        let tops = stacks1.iter().map(|stack| stack.last().unwrap()).fold(String::from(""), |acc, x| format!("{}{}", acc, x).to_owned());

        println!("part 1: {:?}", tops);

        perform_part2(&mut stacks2, &commands);
        let tops = stacks2.iter().map(|stack| stack.last().unwrap()).fold(String::from(""), |acc, x| format!("{}{}", acc, x).to_owned());

        println!("part 2: {:?}", tops);
    });
}

fn perform_part1(stacks: &mut Vec<Vec<char>>, commands: &Vec<(usize, usize, usize)>) {
    for (count, from_idx, to_idx) in commands {
        for _ in 0..*count {
            do_move(stacks, from_idx - 1, to_idx - 1, 1);
        }
    }
}

fn perform_part2(stacks: &mut Vec<Vec<char>>, commands: &Vec<(usize, usize, usize)>) {
    for (count, from_idx, to_idx) in commands {
        do_move(stacks, from_idx - 1, to_idx - 1, *count);
    }
}

fn do_move(stacks: &mut Vec<Vec<char>>, from_idx: usize, to_idx: usize, count: usize) {
    let len = stacks[from_idx].len();
    let to_keep = stacks[from_idx][0..len-count].to_owned();
    let to_move = stacks[from_idx][len-count..].to_owned();

    stacks[from_idx] = to_keep;

    for el in to_move {
        stacks[to_idx].push(el);
    }
}

fn parse_crates(crate_rows: Vec<&str>) -> Vec<Vec<char>> {
    let mut rows = crate_rows
        .iter()
        .map(|row| {
            row.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|c| c[1])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    rows.pop(); // remove unneeded numbers row

    let stacks = (0..rows[0].len())
        .map(|i| {
            rows.iter()
                .map(|inner| inner[i].clone())
                .filter(|el| el.is_alphabetic())
                .rev()
                .collect::<Vec<_>>()
        })
        .collect();

    stacks
}

fn parse_commands(instruction_rows: Vec<&str>) -> Vec<(usize, usize, usize)> {
    instruction_rows
        .iter()
        .map(|row| {
            let parts = row.split(' ').collect::<Vec<&str>>();
            (
                parts[1].parse().unwrap(),
                parts[3].parse().unwrap(),
                parts[5].parse().unwrap(),
            )
        })
        .collect::<_>()
}
