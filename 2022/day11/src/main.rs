use num_bigint::{BigInt, Sign};
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

        let part2 = solve(21, &monkeys, false);
        println!("part 2: {part2}");
    })
}

fn solve(rounds: usize, monkeys: &Vec<Monkey>, do_divide: bool) -> BigInt {
    let mut all_items = monkeys.iter().map(|m| m.items.clone()).collect::<Vec<_>>();
    let mut item_count = vec![0; all_items.len()];

    for idx in 1..(rounds + 1) {
        do_round(&monkeys, &mut all_items, &mut item_count, do_divide);

        if idx == 1 || idx == 20 || idx % 1000 == 0 {
            println!("==== Round {idx} ====");
            // for idx in 0..monkeys.len() {
            //     println!("Monkey {idx}: {:?}", all_items[idx]);
            // }
            for (idx, count) in item_count.iter().enumerate() {
                println!("Monkey {idx} - {count} items");
            }
        }
    }

    item_count.sort();
    item_count
        .iter()
        .rev()
        .take(2)
        .fold(big_int(1), |a, b| a * b)
}

#[derive(Clone)]
struct Monkey {
    items: Vec<BigInt>,
    operation: (Option<BigInt>, Option<BigInt>, char),
    throw_to: (usize, usize, usize),
}

impl Monkey {
    fn get_worry(&self, worry: &BigInt, do_divide: bool) -> BigInt {
        let (left, right, op) = self.operation;
        // let divisor = self.throw_to.0;

        let val: BigInt = match op {
            '+' => left.unwrap_or(*worry) + right.unwrap_or(*worry),
            '-' => left.unwrap_or(*worry) - right.unwrap_or(*worry),
            '*' => left.unwrap_or(*worry) * right.unwrap_or(*worry),
            '/' => left.unwrap_or(*worry) / right.unwrap_or(*worry),
            _ => panic!("unsupported operator: {}", op),
        };

        if do_divide {
            val / 3
        } else {
            val
        }
    }

    fn next(&self, worry: &BigInt) -> usize {
        let (divisor, monkey_true, monkey_false) = self.throw_to;

        if worry % divisor == big_int(0) {
            monkey_true
        } else {
            monkey_false
        }
    }
}

fn do_round(
    monkeys: &Vec<Monkey>,
    all_items: &mut Vec<Vec<BigInt>>,
    item_count: &mut Vec<usize>,
    do_divide: bool,
) {
    for idx in 0..monkeys.len() {
        let m = &monkeys[idx];
        let items = all_items[idx].clone();
        item_count[idx] += items.len();
        for worry in items {
            let new_worry = m.get_worry(&worry, do_divide);
            let next = m.next(&new_worry);
            all_items[next].push(new_worry);
        }
        all_items[idx] = Vec::new();
    }
}

fn get_items(line: &str) -> Vec<BigInt> {
    line.split(": ")
        .last()
        .expect("no starting items found")
        .split(",")
        .map(|x| {
            big_int(
                x.trim()
                    .parse::<usize>()
                    .expect("could not parse item as usize"),
            )
        })
        .collect()
}

fn get_operation(line: &str) -> (Option<BigInt>, Option<BigInt>, char) {
    match sscanf!(line, "  Operation: new = {} {} {}", str, char, str) {
        Ok((left, op, right)) => {
            let left = get_op_val(left);
            let right = get_op_val(right);

            (left, right, op)
        }
        Err(_) => panic!("could not parse operation"),
    }
}

fn get_op_val(arg: &str) -> Option<BigInt> {
    if arg == "old" {
        None
    } else {
        Some(big_int(arg.parse::<usize>().expect("{} not an usize")))
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

fn big_int(n: usize) -> BigInt {
    let bytes = n.to_le_bytes();
    BigInt::from_bytes_le(Sign::Plus, &bytes)
}
