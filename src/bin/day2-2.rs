use std::fs;

fn main() {
    let res = fs::read_to_string("input/day2.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<usize>().expect("Input not a number"))
        })
        .filter(|split| check_list(&split.clone().collect::<Vec<usize>>()))
        .count();

    println!("{res}");
}

fn check_list(list: &[usize]) -> bool {
    let copy = list.to_owned();
    let mut copy_rev = copy.clone();
    copy_rev.reverse();

    if check_list_inc(copy) || check_list_inc(copy_rev) {
        return true;
    }

    for i in 0..list.len() {
        let mut copy = list.to_owned();
        copy.remove(i);
        let mut copy_rev = copy.clone();
        copy_rev.reverse();

        if check_list_inc(copy) || check_list_inc(copy_rev) {
            return true;
        }
    }

    false
}

fn check_list_inc(mut list: Vec<usize>) -> bool {
    let mut last_item = list.pop().expect("At least one input per line!");
    while let Some(now) = list.pop() {
        let diff = now.abs_diff(last_item);

        if !(1..=3).contains(&diff) || now > last_item {
            return false;
        }

        last_item = now;
    }

    true
}
