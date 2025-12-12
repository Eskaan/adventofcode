fn main() {
    // Parse inputs
    let tiles = std::fs::read_to_string("input/2025/day9.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let split = line
                .split(',')
                .map(|n| n.parse::<u64>().expect("invalid input"))
                .collect::<Vec<_>>();
            (split[0], split[1])
        })
        .collect::<Vec<_>>();

    let tile_map = (0..tiles.len())
        .flat_map(|bx| {
            (0..tiles.len())
                .filter(move |&b| b < bx)
                .map(move |b| (bx, b))
        })
        .collect::<Vec<_>>();

    let mut dist_list = Vec::new();
    for &(box_a, box_b) in &tile_map {
        let dist = (tiles[box_a].0.abs_diff(tiles[box_b].0) + 1)
            * (tiles[box_a].1.abs_diff(tiles[box_b].1) + 1);
        dist_list.push(dist);
    }

    println!("max: {}", dist_list.iter().fold(0, |fold, &v| fold.max(v)));
}
