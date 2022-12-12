use sscanf::sscanf;
use util::get_lines;

fn main() {
    get_lines(|lines| {
        let raw_monkeys = lines.chunks(7);

        let monkeys = raw_monkeys
            .map(|monkey| Monkey {
                items: get_items(monkey[1]),
                operation: get_operation(monkey[2]),
                throw_to: get_throw_to(&monkey[3..]),
            })
            .collect::<Vec<Monkey>>();

        let part1 = solve(20, &monkeys, true);
        println!("part 1: {part1}");

        let part2 = solve(10_000, &monkeys, false);
        println!("part 2: {part2}");
    })
}

fn solve(rounds: usize, monkeys: &Vec<Monkey>, do_divide: bool) -> usize {
    let mut all_items = monkeys.iter().map(|m| m.items.clone()).collect::<Vec<_>>();
    let mut item_count = vec![0; all_items.len()];

    for _idx in 1..(rounds + 1) {
        do_round(&monkeys, &mut all_items, &mut item_count, do_divide);

        // if _idx == 1 || _idx == 20 || _idx % 1000 == 0 {
        //     println!("==== Round {_idx} ====");
        //     for idx in 0..monkeys.len() {
        //         println!("Monkey {idx}: {:?}", all_items[idx]);
        //     }
        //     for (idx, count) in item_count.iter().enumerate() {
        //         println!("Monkey {idx} - {count} items");
        //     }
        // }
    }

    item_count.sort();
    item_count.iter().rev().take(2).fold(1, |a, b| a * b)
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: (Option<usize>, Option<usize>, char),
    throw_to: (usize, usize, usize),
}

impl Monkey {
    fn get_worry(&self, worry: usize, divisor: Option<usize>) -> usize {
        let (left, right, op) = self.operation;

        let val: usize = match op {
            '+' => left.unwrap_or(worry) + right.unwrap_or(worry),
            '-' => left.unwrap_or(worry) - right.unwrap_or(worry),
            '*' => left.unwrap_or(worry) * right.unwrap_or(worry),
            '/' => left.unwrap_or(worry) / right.unwrap_or(worry),
            _ => panic!("unsupported operator: {}", op),
        };

        match divisor {
            Some(d) => val % d,
            None => val / 3,
        }
    }

    fn next(&self, worry: usize) -> usize {
        let (divisor, monkey_true, monkey_false) = self.throw_to;

        if worry % divisor == 0 {
            monkey_true
        } else {
            monkey_false
        }
    }
}

fn do_round(
    monkeys: &Vec<Monkey>,
    all_items: &mut Vec<Vec<usize>>,
    item_count: &mut Vec<usize>,
    do_divide: bool,
) {
    let divisor = if do_divide {
        None
    } else {
        Some(monkeys.iter().fold(1, |acc, m| acc * m.throw_to.0))
    };

    for idx in 0..monkeys.len() {
        let m = &monkeys[idx];
        let items = all_items[idx].clone();
        item_count[idx] += items.len();
        for worry in items {
            let new_worry = m.get_worry(worry, divisor);
            let next = m.next(new_worry);
            all_items[next].push(new_worry);
        }
        all_items[idx] = Vec::new();
    }
}

fn get_items(line: &str) -> Vec<usize> {
    line.split(": ")
        .last()
        .expect("no starting items found")
        .split(",")
        .map(|x| {
            x.trim()
                .parse::<usize>()
                .expect("could not parse item as usize")
        })
        .collect()
}

fn get_operation(line: &str) -> (Option<usize>, Option<usize>, char) {
    match sscanf!(line, "  Operation: new = {} {} {}", str, char, str) {
        Ok((left, op, right)) => {
            let left = get_op_val(left);
            let right = get_op_val(right);

            (left, right, op)
        }
        Err(_) => panic!("could not parse operation"),
    }
}

fn get_op_val(arg: &str) -> Option<usize> {
    if arg == "old" {
        None
    } else {
        Some(arg.parse::<usize>().expect("{} not an usize"))
    }
}

fn get_throw_to(lines: &[&str]) -> (usize, usize, usize) {
    let divisor =
        sscanf!(lines[0].trim(), "Test: divisible by {}", usize).expect("no divisor found");
    let monkey_true = sscanf!(lines[1].trim(), "If true: throw to monkey {}", usize)
        .expect("no true monkey found");
    let monkey_false = sscanf!(lines[2].trim(), "If false: throw to monkey {}", usize)
        .expect("no false monkey found");

    (divisor, monkey_true, monkey_false)
}
