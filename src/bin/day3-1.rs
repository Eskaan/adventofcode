use std::fs;

fn main() {
    let mut input = fs::read_to_string("input/day3.txt").unwrap();
    let mut count = 0;

    while !input.is_empty() {
        if input.starts_with("mul(") {
            input.replace_range(0..4, "");
            let num1 = parse_num(&mut input);
            if input.starts_with(',') {
                input.remove(0);
                let num2 = parse_num(&mut input);
                if input.starts_with(')') {
                    input.remove(0);
                    count += num1 * num2;
                }
            }
        } else {
            input.remove(0);
        }
    }

    println!("{count}");
}

fn parse_num(input: &mut String) -> usize {
    let mut buf = String::new();
    while let Some(ch) = input.chars().next() {
        if ch.is_ascii_digit() {
            buf.push(ch);
            input.remove(0);
        } else {
            return buf.parse().unwrap_or(0);
        }
    }

    0
}
