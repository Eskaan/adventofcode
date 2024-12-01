use std::fs;

fn main() {
    let mut list1 = Vec::<usize>::new();
    let mut list2 = Vec::<usize>::new();

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

    let res = list1.into_iter().fold(0, |fold, input| {
        fold + (input * list2.iter().filter(|i| input == **i).count())
    });

    println!("{res}");
}
