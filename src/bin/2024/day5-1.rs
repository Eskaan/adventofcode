use std::fs;

fn main() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|l| l.split_once('|').unwrap())
        .map(|(src, dest)| (src.parse().unwrap(), dest.parse().unwrap()))
        .collect::<Vec<(usize, usize)>>();
    let updates = updates.lines().map(|l| {
        l.split(',')
            .map(|page| page.parse().unwrap())
            .collect::<Vec<usize>>()
    });

    let mut count = 0;
    'update: for update in updates {
        for (i, page) in update.iter().enumerate() {
            for (_, before) in rules.iter().filter(|(c, _)| c == page).copied() {
                if let Some(pos) = update.iter().position(|c| *c == before) {
                    if pos < i {
                        continue 'update;
                    }
                }
            }
        }
        assert_eq!(update.len() % 2, 1);
        count += update[update.len() / 2];
    }
    println!("{count}");
}
