fn main() {
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|machine| {
            let lines: Vec<_> = machine
                .lines()
                .map(|l| {
                    let x = l.find('X').unwrap() + 2;
                    let y = l.find('Y').unwrap() + 2;
                    (l[x..(y - 4)].parse().unwrap(), l[y..].parse().unwrap())
                })
                .collect();
            Machine {
                a: lines[0],
                b: lines[1],
                prize: lines[2],
            }
        })
        .collect();

    let mut counter = 0;
    for m in machines {
        println!("{:?}", m);
        let mut best = 0;
        for b in 0..=100 {
            if b * m.b.0 > m.prize.0 || b * m.b.1 > m.prize.1 {
                println!("{b} too high");
                break;
            }
            /*println!(
                "{} {} {}",
                b,
                (m.prize.0 - (b * m.b.0)),
                (m.prize.0 - (b * m.b.0)) % m.a.0
            );*/
            if (m.prize.0 - (b * m.b.0)) % m.a.0 == 0 && (m.prize.1 - (b * m.b.1)) % m.a.1 == 0 {
                let a = (m.prize.0 - (b * m.b.0)) / m.a.0;
                if a > 100 {
                    continue;
                }
                let tokens = a + (b * 3);
                println!("Solution found {} {} = {}", b, a, tokens);
                if tokens < best || best == 0 {
                    best = tokens;
                }
            }
        }
        counter += best;
    }
    println!("{counter}");
}

#[derive(Debug, Copy, Clone)]
pub struct Machine {
    pub a: (usize, usize),
    pub b: (usize, usize),
    pub prize: (usize, usize),
}
