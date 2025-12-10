fn main() {
    let mut counter = 0;

    for line in std::fs::read_to_string("input/2025/day10.txt")
        .unwrap()
        .trim()
        .lines()
    {
        let line_split = line
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let lights = line_split[0]
            .chars()
            .filter_map(|l| match l {
                '.' => Some(0),
                '#' => Some(1),
                _ => None,
            })
            .enumerate()
            .fold(0, |fold, (i, b)| fold + (b << i));

        let buttons = line_split[1..line_split.len() - 1]
            .iter()
            .map(|s| {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|num| num.parse::<u64>().unwrap())
                    .fold(0, |fold, i| fold + (1 << i))
            })
            .collect::<_>();

        let num_press = solve(lights, buttons);
        println!("{line} => {num_press}");
        counter += num_press;
    }

    println!("counter: {counter}");
}

fn solve(lights: u64, buttons: Vec<u64>) -> usize {
    print!("Solve for {lights:b} - ");
    for btn in &buttons {
        print!("{btn:b} ");
    }
    println!();

    let mut cmb: Vec<usize> = vec![0];
    let mut cur = 0;
    while cur != lights {
        cur = 0;
        for i in &cmb {
            cur ^= buttons[*i];
        }
        if cur == lights {
            break;
        }
        //println!("cur: {cur:b} cmb: {cmb:?}");

        // Count up cmb
        for i in 0..cmb.len() {
            if cmb[i] == buttons.len() - 1 {
                cmb[i] = 0;
                if i == cmb.len() - 1 {
                    cmb.push(0);
                }
            } else {
                cmb[i] += 1;
                break;
            }
        }
    }

    return cmb.len();
}
