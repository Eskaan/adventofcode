fn main() {
    // Parse inputs
    let directions = std::fs::read_to_string("input/2025/day1.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let val = line[1..].parse::<i64>().unwrap();
            match line.chars().nth(0).unwrap() {
                'R' => val,
                'L' => -val,
                _ => panic!("Invalid input"),
            }
        })
        .collect::<Vec<_>>();

    // Math
    let mut dial = 50;
    let mut counter = 0;
    for i in directions {
        let last = dial;

        dial += i;
        println!("{counter} R: {last} -> {dial} D: {i}");
        if dial.is_positive() {
            if last.is_negative() {
                counter += 1;
            }
            counter += dial.abs() / 100;
            dial %= 100;
        } else if dial.is_negative() {
            if last.is_positive() {
                counter += 1;
            }
            counter += dial.abs() / 100;
            dial %= 100;
        } else if dial == 0 {
            counter += i / 100;
            counter += 1;
        } else {
            unreachable!()
        }
    }

    // Output
    println!("{counter}");
}
