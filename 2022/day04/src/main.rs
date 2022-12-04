use util::get_lines;

fn main() {
    get_lines(|lines| {
        let section_pairs = lines.iter().map(|line| {
            let sections = line
                .split(',')
                .map(|part| {
                    let parts = part
                        .split('-')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();

                    (parts[0], parts[1])
                })
                .collect::<Vec<(usize, usize)>>();

            (sections[0], sections[1])
        });

        let ranges_contained = section_pairs
            .to_owned()
            .filter(|pair| contains_range(pair.0, pair.1));

        let ranges_overlapped = section_pairs.filter(|pair| overlaps(pair.0, pair.1));

        println!("part 1: {:?}", ranges_contained.collect::<Vec<_>>().len());
        println!("part 2: {:?}", ranges_overlapped.collect::<Vec<_>>().len());
    });
}

fn contains_range(range1: (usize, usize), range2: (usize, usize)) -> bool {
    (range1.0 <= range2.0 && range1.1 >= range2.1) || (range2.0 <= range1.0 && range2.1 >= range1.1)
}

fn overlaps(range1: (usize, usize), range2: (usize, usize)) -> bool {
    let a = range1.0 <= range2.0 && range1.1 >= range2.0;
    let b = range1.0 <= range2.1 && range1.1 >= range2.1;
    let c = range2.0 <= range1.0 && range2.1 >= range1.0;
    let d = range2.0 <= range1.1 && range2.1 >= range1.1;

    a || b || c || d
}
