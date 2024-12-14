fn main() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    let width: isize = 101;
    let height: isize = 103;
    let steps = 100;
    let robots: Vec<((isize, isize), (isize, isize))> = input
        .lines()
        .map(|l| {
            let split: Vec<isize> = l[2..]
                .split(" v=")
                .map(|s| s.split(','))
                .flatten()
                .map(|s| s.parse().unwrap())
                .collect();
            ((split[0], split[1]), (split[2], split[3]))
        })
        .collect();

    let mut target: Vec<(isize, isize)> = Vec::new();
    for &(pos, v) in &robots {
        let x = (v.0 * steps + pos.0) % width;
        let y = (v.1 * steps + pos.1) % height;
        println!("in: {} {} out: {} {}", pos.0, pos.1, x, y);
        target.push((x, y));
    }

    target = target
        .iter()
        .map(|(mut x, mut y)| {
            if x.is_negative() {
                x += width
            }
            if y.is_negative() {
                y += height
            }
            x -= width / 2;
            y -= height / 2;
            (x, y)
        })
        .collect();
    debug_print(&target, width/2, height/2);

    let mut quads = [0, 0, 0, 0];
    for (x, y) in target {
        if x != 0 && y != 0 {
            match (x.is_positive(), y.is_positive()) {
                (true, true) => quads[0] += 1,
                (true, false) => quads[1] += 1,
                (false, true) => quads[2] += 1,
                (false, false) => quads[3] += 1,
            }
        }
    }
    println!("{quads:?}");
    let count = quads.into_iter().fold(1, |f, q| f * q);
    println!("{count}")
}

fn debug_print(target: &Vec<(isize, isize)>, width: isize, height: isize) {
    for y in -height..=height {
        for x in -width..=width {
            let count = target.iter().filter(|p| **p == (x, y)).count();
            if count > 0 {
                print!("{count}")
            } else {
                match (x, y) {
                    (0, 0) => print!("+"),
                    (0, _) => print!("|"),
                    (_, 0) => print!("-"),
                    _ => print!("."),
                }
            }
        }

        println!()
    }
    println!()
}
