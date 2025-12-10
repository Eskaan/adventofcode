fn main() {
    // Parse inputs
    let map = std::fs::read_to_string("input/2025/day4.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => 0,
                    '@' => 1,
                    _ => unreachable!("Invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut counter = 0;

    for y in 0..map.len() {
        for x in 0..map.len() {
            if x == 0 {
                println!();
            }
            if map[y][x] == 0 {
                print!(".");
                continue;
            }

            let neighbors = get_from_map(&map, x, y, -1, -1)
                + get_from_map(&map, x, y, 0, -1)
                + get_from_map(&map, x, y, 1, -1)
                + get_from_map(&map, x, y, -1, 0)
                + get_from_map(&map, x, y, 1, 0)
                + get_from_map(&map, x, y, -1, 1)
                + get_from_map(&map, x, y, 0, 1)
                + get_from_map(&map, x, y, 1, 1);
            //println!("n: {neighbors} {}", neighbors < 4);
            if neighbors < 4 {
                print!("x");
                counter += 1;
            } else {
                print!("@");
            }
        }
    }

    println!("counter: {counter}");
}

fn get_from_map(map: &Vec<Vec<usize>>, x: usize, y: usize, nx: isize, ny: isize) -> usize {
    *match map.get(match y.checked_add_signed(ny) {
        None => return 0,
        Some(y) => y,
    }) {
        None => return 0,
        Some(line) => line.get(match x.checked_add_signed(nx) {
            None => return 0,
            Some(x) => x,
        }),
    }
    .unwrap_or(&0)
}
