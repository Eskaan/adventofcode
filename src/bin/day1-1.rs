use std::fs;

fn main() {
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    // Parse inputs
    fs::read_to_string("input/day1.txt")
        .unwrap()
        .lines()
        .map(|l| l.split("   ").collect::<Vec<_>>())
        .for_each(|split| {
            assert_eq!(split.len(), 2);
            list1.push(split[0].parse().unwrap());
            list2.push(split[1].parse().unwrap());
        });

    // Actual Math

    list1.sort();
    list2.sort();

    let res = list1
        .into_iter()
        .zip(list2.into_iter())
        .fold(0, |fold, (i1, i2)| {
            println!("{} {} diff {}", i1, i2, (i1 - i2).abs());
            fold + (i1 - i2).abs()
        });

    println!("{res}");
}
