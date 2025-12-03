fn main() {
    // Parse inputs
    let batteries = std::fs::read_to_string("input/2025/day3.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.chars().map(|ch| ch as usize - '0' as usize ).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counter = 0;

    for bat in batteries {
        let mut max = 0;
        let mut max_index = 0;

        for digit in (0..12).rev() {
            let mut max_cur = 0;
            for i in max_index..bat.len()-digit {
                if bat[i] > max_cur {
                    max_cur = bat[i];
                    max_index = i;
                }
            }
            max += max_cur * 10usize.pow(digit as u32);
            max_index += 1;
        }

        counter += max;
    }
    println!("counter: {counter}");
}
