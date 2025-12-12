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

    let nums1 = &lines[0];
    let nums2 = &lines[1];
    let nums3 = &lines[2];
    let nums4 = &lines[3];

    let mut counter = 0;

    for (i, &(pos, symbol)) in symbols.iter().enumerate() {
        let pos_end = symbols.get(i + 1).unwrap_or(&(nums1.len() + 1, ' ')).0 - 1;
        //let pos_end = symbols.get(i+1).unwrap().0 - 1;
        let num1: usize = nums1[pos..pos_end].trim().parse().unwrap();
        let num2: usize = nums2[pos..pos_end].trim().parse().unwrap();
        let num3: usize = nums3[pos..pos_end].trim().parse().unwrap();
        let num4: usize = nums4[pos..pos_end].trim().parse().unwrap();

        let res = match symbol {
            '+' => num1 + num2 + num3 + num4,
            '*' => num1 * num2 * num3 * num4,
            _ => unreachable!("Invalid input"),
        };
        println!("res: {res}");
        counter += res;
    }

    println!("counter: {counter}");
}
