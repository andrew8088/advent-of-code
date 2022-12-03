use util::get_lines;

const LOWERCASE_OFFSET: u8 = b'a' - 1;
const UPPERCASE_OFFSET: u8 = b'A' - 27;

fn main() {
    // part 1
    get_lines(|lines| {
        let pairs = lines.iter().map(|line| {
            let mid = line.len() / 2;
            let (a, b) = line.split_at(mid);
            Vec::from([a.to_owned(), b.to_owned()])
        });

        let sum: usize = pairs.map(find_common_item).map(to_priority).sum();

        println!("part 1: {}", sum);
    });

    // part 2
    get_lines(|lines| {
        let groups = lines.chunks(3).map(|chunk| {
            chunk
                .iter()
                .map(|s| s.to_owned().to_owned())
                .collect::<Vec<String>>()
        });

        let sum: usize = groups.map(find_common_item).map(to_priority).sum();

        println!("part 2: {}", sum);
    });
}

fn find_common_item(sacks: Vec<String>) -> char {
    match sacks.split_first() {
        Some((first, rest)) => {
            'outer: for c in first.chars() {
                for v in rest {
                    if !v.contains(c) {
                        continue 'outer;
                    }
                }
                return c;
            }
            char::from(LOWERCASE_OFFSET)
        }
        None => char::from(LOWERCASE_OFFSET),
    }
}

fn to_priority(c: char) -> usize {
    let offset = if c.is_uppercase() {
        UPPERCASE_OFFSET
    } else {
        LOWERCASE_OFFSET
    };

    let prio = (c as u8) - offset;
    return prio.into();
}
