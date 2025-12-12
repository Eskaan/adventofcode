fn main() {
    // Parse inputs
    let lines = std::fs::read_to_string("input/2025/day7.txt")
        .unwrap()
        .trim_end()
        .lines()
        .map(str::to_string)
        .collect::<Vec<_>>();

    let mut beams = Vec::new();
    beams.push(lines[0].find('S').unwrap());

    let mut counter = 0;

    for line in &lines[1..] {
        for i in 0..beams.len() {
            if line.as_bytes()[beams[i]] == b'^' {
                beams.push(beams[i] + 1);
                beams[i] -= 1;
                counter += 1;
            }
        }

        beams.sort();
        beams.dedup();
        println!("{beams:?}");
    }

    println!("counter: {counter}");
}
