use regex::{Regex, Match};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
enum Op {
    VALUE,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
}

#[derive(Debug, Copy, Clone)]
enum Value {
    VAR([char;2]),
    RAW(u16),
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    name: Op,
    right: Option<Value>,
    left: Option<Value>,
    assignment: [char;2],
}

fn main() {
    let input = get_input();
    let parts = input.trim().split("\n");

    let mut ops: VecDeque<Operation> = VecDeque::new();
    for part in parts {
        match parse_line(part) {
            Some(op) => {
                ops.push_back(op);
            }
            None => {}
        }
    }

    // part 1
    let vars = exec_ops(ops.clone(), HashMap::new());
    let a_val = vars[&['a', ' ']];
    println!("a: {:?}", a_val);

    // part 2
    let new_ops = remove_assignment(ops, ['b', ' ']);
    let init_vars = HashMap::from([(['b', ' '], a_val)]);

    let vars = exec_ops(new_ops, init_vars);
    println!("a: {:?}", vars[&['a', ' ']]);
}

fn remove_assignment(ops: VecDeque<Operation>, assignment: [char;2]) -> VecDeque<Operation> {
    let mut new_ops: VecDeque<Operation> = VecDeque::new();

    for op in ops {
        if op.assignment != assignment {
            new_ops.push_back(op);
        }
    }

    new_ops
}

fn exec_ops(mut ops: VecDeque<Operation>, mut vars: HashMap<[char;2], u16>) -> HashMap<[char;2], u16> {
    while ops.len() > 0 {
        let op  = ops.pop_front().unwrap();
        let assignment = op.assignment;
        // println!("op: {:?}", op);

        let right = get_raw_val(op.right, &vars);
        let left = get_raw_val(op.left, &vars);
        
        let new_value = match op.name {
            Op::VALUE => right,
            Op::NOT => left.map(|l| !l),
            Op::LSHIFT => perform_op(right, left, |r,l| r << l),
            Op::RSHIFT => perform_op(right, left, |r,l| r >> l),
            Op::AND => perform_op(right, left, |r,l| r & l),
            Op::OR => perform_op(right, left, |r,l| r | l),
        };

        match new_value {
            Some(new_value) => {
                vars.insert(assignment, new_value);
            }
            None => {
                ops.push_back(op);
            }
        }
    }

    vars
}

fn perform_op<F : Fn(u16, u16) -> u16>(right: Option<u16>, left: Option<u16>, f: F) -> Option<u16> {
    match (right, left) {
        (Some(right_val), Some(left_val)) => Some(f(right_val, left_val)),
        _ => None
    }
}

fn get_raw_val(val: Option<Value>, vars: &HashMap<[char;2], u16>) -> Option<u16> {
    match val {
        Some(val) => {
            match val {
                Value::RAW(raw_val) => Some(raw_val),
                Value::VAR(var_name) => {
                    match vars.get(&var_name) {
                        Some(raw_val) => Some(*raw_val),
                        None => None
                    }
                }
            }
        }
        None => None
    }
}

fn to_op(op_name: Option<Match>) -> Op {
    match op_name {
        Some(op_name) => {
            match op_name.as_str() {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "LSHIFT" => Op::LSHIFT,
                "RSHIFT" => Op::RSHIFT,
                "NOT" => Op::NOT,
                _ => Op::VALUE,
            }
        }
        None => Op::VALUE,
    }
}

fn to_char2(s: &str) -> [char;2] {
    let mut chars = s.chars();
    let c1 = chars.next().or(Some(' ')).unwrap();
    let c2 = chars.next().or(Some(' ')).unwrap();
    [c1, c2]
}

fn to_value(val: Option<Match>) -> Option<Value> {
    match val {
        Some(val) => {
            let val_str: &str = val.as_str();
            match val_str.parse::<u16>() {
                Ok(num1) => Some(Value::RAW(num1)),
                Err(_) => Some(Value::VAR(to_char2(val_str)))
            }
        },
        None => None
    }
}

fn parse_line(line: &str) -> Option<Operation> {
    let top_re = Regex::new(r"([0-9a-z]+)? ?([A-Z]+)? ?([0-9a-z]+)? -> (.+)").unwrap();

    match top_re.captures(line) {
        Some(caps) => {
            let right = to_value(caps.get(1));
            let name = to_op(caps.get(2));
            let left = to_value(caps.get(3));
            let assignment = to_char2(caps.get(4).unwrap().as_str());

            return Some(Operation {
                name,
                right,
                left,
                assignment,
            });
        },
        None => {
            return None;
        }
    }
}

// fn get_test_input() -> String {
//     return String::from("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i");
// }

fn get_input() -> String {
    let filename = "./input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            return content;
        },
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
            return String::from("");
        },
    }
}
