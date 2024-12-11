//! Note that this is efectively the same code as day11-1, just with the loop increased to 75

use std::collections::BTreeMap;

fn main() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    let mut nums: BTreeMap<u32, u32> = BTreeMap::new();
    input.split(' ').for_each(|n| {
        nums.insert(n.parse().unwrap(), 1);
    });

    for _ in 0..75 {
        let mut rebuild = BTreeMap::new();
        for (num, count) in nums {
            if num == 0 {
                *rebuild.entry(1).or_default() += count;
            } else if (num.ilog10() + 1) % 2 == 0 {
                let split = 10u32.pow((num.ilog10() / 2) + 1);
                *rebuild.entry(num / split).or_default() += count;
                *rebuild.entry(num % split).or_default() += count;
            } else {
                *rebuild.entry(num * 2024).or_default() += count;
            }
        }
        nums = rebuild;
    }

    println!("{}", nums.values().fold(0, |f, n| f + n));
}
