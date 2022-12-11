use sscanf::sscanf;
use util::get_lines;

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: (Option<usize>, Option<usize>, char),
    throw_to: (usize, usize, usize),
}

impl Monkey {
    fn get_worry(&self, worry: usize) -> usize {
        let (left, right, op) = self.operation;

        match op {
            '+' => left.unwrap_or(worry) + right.unwrap_or(worry),
            '-' => left.unwrap_or(worry) - right.unwrap_or(worry),
            '*' => left.unwrap_or(worry) * right.unwrap_or(worry),
            '/' => left.unwrap_or(worry) / right.unwrap_or(worry),
            _ => panic!("unsupported operator: {}", op),
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
        let mut all_items = monkeys.iter().map(|m| m.items.clone()).collect::<Vec<_>>();

        for round in 1..21 {
            println!("Round {}", round);

            for idx in 0..monkeys.len() {
                // println!("monkey {idx}");
                let m = &monkeys[idx];
                let items = all_items[idx].clone();
                for worry in items {
                    let new_worry = m.get_worry(worry) / 3;
                    let next = m.next(new_worry);
                    // println!("{} => {} (throw to {})", worry, new_worry, next,);
                    all_items[next].push(new_worry);
                }
                all_items[idx] = Vec::new();
            }

            for idx in 0..all_items.len() {
                println!("Monkey {idx}: {:?}", all_items[idx]);
            }
        }
    })
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

    // move |value: usize| {
    //     if value % divisor == 0 {
    //         monkey_true
    //     } else {
    //         monkey_false
    //     }
    // }
}
