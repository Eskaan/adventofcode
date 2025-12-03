fn main() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    let width: isize = 101;
    let height: isize = 103;
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

    let mut i = 0;
    loop {
        let mut target: Vec<(isize, isize)> = Vec::new();
        for &(pos, v) in &robots {
            let x = (v.0 * i + pos.0) % width;
            let y = (v.1 * i + pos.1) % height;
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

        write_to_file(&target, i, width / 2, height / 2);
        i += 1;
    }
}

fn write_to_file(target: &Vec<(isize, isize)>, i: isize, width: isize, height: isize) {
    let mut s = String::with_capacity((width * height + height) as usize);
    for y in -height..=height {
        for x in -width..=width {
            let count = target.iter().filter(|p| **p == (x, y)).count();
            if count > 0 {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    std::fs::write(format!("out/{i}.txt"), s).unwrap();
}
