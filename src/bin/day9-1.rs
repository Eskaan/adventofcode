fn main() {
    let input = std::fs::read_to_string("input/day9.txt").unwrap();

    let mut layout = Vec::new();
    let mut read_space = false;
    let mut id: u16 = 0;

    for i in input.chars().map(|ch| ch as u8 - b'0') {
        print!("{i}");
        if read_space {
            for _ in 0..i {
                layout.push(None);
            }
        } else {
            for _ in 0..i {
                layout.push(Some(id));
            }
            id = id.checked_add(1).unwrap();
        }

        read_space = !read_space;
    }
    println!();

    let mut i = 0;
    while let Some(opt) = layout.get(i) {
        if opt.is_none() {
            loop {
                if let Some(Some(id)) = layout.pop() {
                    if let Some(wrt) = layout.get_mut(i) {
                        *wrt = Some(id);
                    } else {
                        println!("Bounds");
                    }
                    break;
                }
            }
        }
        i += 1;
    }

    let mut count: usize = 0;
    for (i, opt_id) in layout.iter().enumerate() {
        if let Some(id) = opt_id {
            count = count.checked_add(i * *id as usize).unwrap();
        } else {
            println!("id none");
        }
    }
    println!("{count}");
}
