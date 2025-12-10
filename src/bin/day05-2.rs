fn main() {
    // Parse inputs
    let (ranges_str, ids_str) = std::fs::read_to_string("input/2025/day5.txt")
        .unwrap()
        .trim()
        .split_once("\n\n")
        .map(|(r, i)| (r.to_string(), i.to_string()))
        .unwrap();

    let ranges = ranges_str.lines().map(|line| {
        let (from, to) = line.split_once('-').unwrap();
        from.parse().unwrap()..=to.parse::<u64>().unwrap()
    });

    let ids = ids_str.lines().map(|id| id.parse().unwrap()).collect::<Vec<u64>>();

    let mut fresh = 0;

    for id in ids {
        for range in ranges.clone() {
            println!("id: {id} range: {range:?}");
            if range.contains(&id) {
                println!("fresh!");
                fresh += 1;
                break;
            }
        }
    }

    println!("fresh: {fresh}");
}
