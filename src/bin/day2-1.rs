use std::fs;

fn main() {
    let res = fs::read_to_string("input/day2.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|s| s.parse::<usize>().expect("Input not a number"))
        })
        .map(|mut split| {
            let mut inc = None;
            let mut last = split.next().expect("At least one input per line!");
            while let Some(now) = split.next() {
                if now > last && now - last <= 3 {
                    match inc {
                        None => inc = Some(true),
                        Some(true) => {}
                        Some(false) => return false,
                    }
                } else if now < last && last - now <= 3 {
                    match inc {
                        None => inc = Some(false),
                        Some(true) => return false,
                        Some(false) => {}
                    }
                } else {
                    return false;
                }

                last = now;
            }

            return true;
        })
        .filter(|res| *res)
        .count();

    println!("{res}");
}
