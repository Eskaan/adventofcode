fn main() {
    // Parse inputs
    let boxes = std::fs::read_to_string("input/2025/day8.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let split = line
                .split(',')
                .map(|n| n.parse::<u64>().expect("invalid input"))
                .collect::<Vec<_>>();
            (split[0], split[1], split[2])
        })
        .collect::<Vec<_>>();

    let mut box_map = (0..boxes.len())
        .flat_map(|bx| {
            (0..boxes.len())
                .filter(move |&b| b < bx)
                .map(move |b| (bx, b))
        })
        .collect::<Vec<_>>();

    let mut dist_list = Vec::new();

    println!("Sorting by distance...");
    box_map.sort_by_cached_key(|&(box_a, box_b)| {
        let dist = boxes[box_a].0.abs_diff(boxes[box_b].0).pow(2)
            + boxes[box_a].1.abs_diff(boxes[box_b].1).pow(2)
            + boxes[box_a].2.abs_diff(boxes[box_b].2).pow(2);
        dist_list.push(dist);
        dist
    });

    let len_full = dist_list.len();
    dist_list.sort();
    dist_list.dedup();
    let len_dedup = dist_list.len();
    assert_eq!(len_dedup, len_full);

    println!("Merging...");
    let mut circuits: Vec<Vec<usize>> = (0..boxes.len()).map(|id| vec![id]).collect();

    for (box_a, box_b) in box_map {
        let merge = circuits
            .iter_mut()
            .filter(|crt| crt.contains(&box_a) || crt.contains(&box_b))
            .fold(Vec::new(), |mut fold, crt| {
                fold.append(crt);
                fold
            });
        circuits.push(merge);

        circuits.retain(|crt| crt.len() > 0);
        if circuits.len() == 1 {
            println!("fin {box_a} {box_b}");
            println!("sum: {}", boxes[box_a].0 * boxes[box_b].0);
            break;
        }
    }
}
