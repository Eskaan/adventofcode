fn main() {
    // Parse inputs
    let batteries = std::fs::read_to_string("input/2025/day3.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch as u64 - '0' as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut counter = 0;

    for bat in batteries {
        let mut first_max = 0;
        let mut index = 0;
        for i in 0..bat.len() - 1 {
            if bat[i] > first_max {
                first_max = bat[i];
                index = i;
            }
        }

        let mut second_max = 0;
        for i in index + 1..bat.len() {
            if bat[i] > second_max {
                second_max = bat[i];
            }
        }

        let max = first_max * 10 + second_max;
        counter += max;
    }
    println!("counter: {counter}");
}
