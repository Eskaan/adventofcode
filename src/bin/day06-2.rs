fn main() {
    // Parse inputs
    let mut lines = std::fs::read_to_string("input/2025/day6.txt")
        .unwrap()
        .trim_end()
        .lines()
        .map(str::to_string)
        .collect::<Vec<_>>();

    // Last line is symbols
    let symbols = lines
        .pop()
        .unwrap()
        .char_indices()
        .filter(|(_, ch)| *ch != ' ')
        .collect::<Vec<_>>();

    let mut counter: usize = 0;

    for (i, &(pos, symbol)) in symbols.iter().enumerate() {
        let pos_end = symbols.get(i + 1).unwrap_or(&(lines[0].len() + 1, ' ')).0 - 1;
        //let pos_end = symbols.get(i+1).unwrap().0 - 1;
        let mut res = match symbol {
            '+' => 0,
            '*' => 1,
            _ => unreachable!("Invalid input"),
        };
        for i in pos..pos_end {
            let mut num_str = String::new();
            for line in &lines {
                num_str.push(line.chars().nth(i).unwrap());
            }

            println!("str: {num_str}");
            let num = num_str.trim().parse::<usize>().unwrap();
            match symbol {
                '+' => res += num,
                '*' => res *= num,
                _ => unreachable!("Invalid input"),
            };
        }

        println!("res: {res}");
        counter += res;
    }

    println!("counter: {counter}");
}
