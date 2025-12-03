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
    for update in updates {
        let mut reorderd = update.clone();
        let mut has_changed = true;

        while has_changed {
            has_changed = false;
            'update: for (i, page) in reorderd.clone().iter().enumerate() {
                for (_, before) in rules.iter().filter(|(c, _)| c == page).copied() {
                    if let Some(pos) = reorderd.iter().position(|c| *c == before) {
                        if pos < i {
                            has_changed = true;
                            reorderd.remove(i);
                            reorderd.insert(pos, *page);
                            break 'update;
                        }
                    }
                }
            }
        }
        if update != reorderd {
            assert_eq!(reorderd.len() % 2, 1);
            count += reorderd[reorderd.len() / 2];
        }
    }
    println!("{count}");
}
