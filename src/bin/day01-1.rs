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

    let mut dial = 50;
    let mut counter = 0;
    for dir in directions {
        dial += dir;
        if dial % 100 == 0 {
            counter += 1;
        }
    }
    println!("{counter}");
}
