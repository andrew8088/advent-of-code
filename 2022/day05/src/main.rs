use util::get_lines;

fn main() {
    get_lines(|lines| {
        let mut iter = lines.iter();
        let crates = iter
            .by_ref()
            .take_while(|l| !l.trim().is_empty()) // empty line will be consumed here
            .cloned()
            .collect::<Vec<_>>();

        let commands = iter.cloned().collect::<Vec<_>>();

        let stacks = parse_crates(crates);
        let commands = parse_commands(commands);

        perform(&stacks, commands);

        println!("{:?}", stacks);
    });
}

fn perform(stacks: &Vec<Vec<char>>, commands: Vec<(usize, usize, usize)>) {}

fn do_move(stacks: &Vec<Vec<char>>, from_idx: usize, to_idx: usize) {
    let from = &stacks[from_idx];
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
                // .rev()
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
