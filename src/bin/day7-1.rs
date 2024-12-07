fn main() {
    let input = std::fs::read_to_string("input/day7.txt").expect("Could not open input file");

    let equasions: Vec<(usize, Vec<usize>)> = input
        .lines()
        .map(|line| {
            let (res, nums) = line.split_once(": ").expect("Result separator missing");
            let res = res.parse().expect("Not a number");
            let nums = nums
                .split(' ')
                .map(|num| num.parse().expect("Not a number"))
                .collect();
            (res, nums)
        })
        .collect();

    let mut count = 0;
    for (result, mut nums) in equasions {
        if count_recursive(result, nums.remove(0), &nums) {
            count += result;
        }
    }
    println!("{count}");
}

/// Due to optimizations doesn't work with 0 as multiplication (this is the only way a number can get smaller)
fn count_recursive(expected: usize, current: usize, nums: &[usize]) -> bool {
    if expected < current {
        return false;
    }
    if nums.len() <= 1 {
        return expected == current + nums[0] || expected == current * nums[0];
    }
    return count_recursive(expected, current + nums[0], &nums[1..])
        || count_recursive(expected, current * nums[0], &nums[1..]);
}
