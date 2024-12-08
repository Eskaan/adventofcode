use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/day8.txt").unwrap();
    let size = 50;
    // Note that the matrix is read in with (y, x)
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.char_indices()
            .filter(|(_, ch)| *ch != '.')
            .for_each(|(ii, ch)| {
                if !antennas.contains_key(&ch) {
                    antennas.insert(ch, Vec::new());
                }
                antennas
                    .get_mut(&ch)
                    .unwrap()
                    .push((i as isize, ii as isize));
            })
    });

    let mut antinodes: Vec<(isize, isize)> = Vec::new();
    for am in antennas.values() {
        for i in 0..am.len() {
            let fx = am[i].0;
            let fy = am[i].1;

            for ii in (0..am.len()).filter(|ii| *ii != i) {
                let sx = am[ii].0;
                let sy = am[ii].1;

                let dx = fx - sx;
                let dy = fy - sy;

                let ax = fx + dx;
                let ay = fy + dy;
                if ax >= 0 && ax < size && ay >= 0 && ay < size {
                    if ax != sx || ay != sy {
                        antinodes.push((ax, ay));
                    }
                }

                let ax = fx - dx;
                let ay = fy - dy;
                if ax >= 0 && ax < size && ay >= 0 && ay < size {
                    if ax != sx || ay != sy {
                        antinodes.push((ax, ay));
                    }
                }
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();

    println!("{}", antinodes.len());
}
