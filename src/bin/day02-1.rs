fn main() {
    // Parse inputs
    let ranges = std::fs::read_to_string("input/2025/day2.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|l| {
            let mut split = l.split("-");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(u64, u64)>>();

    let mut counter = 0;

    for range in ranges {
        for i in range.0..=range.1 {
            if run(i) {
                counter += i;
            }
        }
    }
    println!("counter: {counter}");
}

fn run(num: u64) -> bool {
    let len = num.ilog10();
    if len % 2 == 0 {
        return false;
    }
    let shift = 10u64.pow(len / 2 + 1);
    let left = num / shift;
    let right = num % shift;
    if right == left {
        return true;
    } else {
        return false;
    }
}
