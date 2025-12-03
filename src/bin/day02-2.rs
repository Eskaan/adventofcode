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
            if pattern(i) {
                counter += i;
            }
        }
    }
    println!("counter: {counter}");
}

fn pattern(num: u64) -> bool {
    let len = num.ilog10() + 1;
    'variations: for i in 2..=len {
        if len % i == 0 {
            let shift = 10u64.pow(len / i);
            let mut cp = num / shift;
            let pattern = num % shift;
            while cp > 0 {
                let right = cp % shift;
                if right != pattern {
                    continue 'variations;
                }
                cp = cp / shift;
            }
            return true;
        }
    }
    return false;
}
