use std::fs;

fn main() {
    let res = fs::read_to_string("input/day2.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<usize>().expect("Input not a number"))
        })
        .filter(|split| check_list(split.clone().collect()))
        .count();

    println!("{res}");
}

fn check_list(list: Vec<usize>) -> bool {
    let copy = list.clone();
    let mut copy_rev = copy.clone();
    copy_rev.reverse();

    if check_list_inc(copy) || check_list_inc(copy_rev) {
        return true;
    }

    for i in 0..list.len() {
        let mut copy = list.clone();
        copy.remove(i);
        let mut copy_rev = copy.clone();
        copy_rev.reverse();

        if check_list_inc(copy) || check_list_inc(copy_rev) {
            return true;
        }
    }

    return false;
}

fn check_list_inc(mut list: Vec<usize>) -> bool {
    let mut last = list.pop().expect("At least one input per line!");
    while let Some(now) = list.pop() {
        let diff = if now > last { now - last } else { last - now };

        if diff > 3 || diff < 1 || now > last {
            return false;
        }

        last = now;
    }

    return true;
}
