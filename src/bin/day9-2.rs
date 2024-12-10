fn main() {
    let input = std::fs::read_to_string("input/day9.txt").unwrap();

    let mut layout = Vec::new();
    let mut read_space = false;
    let mut id: u16 = 0;

    for i in input.chars().map(|ch| ch as u8 - b'0') {
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

    #[rustfmt::skip]
    println!("{}", layout.iter().map(|opt| opt.map(|id| ((id % 10) as u8 + b'0') as char).unwrap_or('.')).collect::<String>());

    // Use last assigned id
    let mut count_id = None;
    let mut layout_count = 0;
    for layout_index in (0..layout.len()).rev() {
        if layout[layout_index] == count_id {
            layout_count += 1;
        } else {
            if count_id.is_some() {
                let mut space_count = 0;
                for (space_index, id) in layout.iter().enumerate().take(layout_index - layout_count)
                {
                    if id.is_some() {
                        if space_count >= layout_count {
                            for i in 0..layout_count {
                                let src = layout_index + 1 + i;
                                let dst = (space_index + i) - space_count;
                                println!(
                                    "Swap {} {:?} - {} {:?}",
                                    src, layout[src], dst, layout[dst]
                                );
                                if src < dst {
                                    println!("Noop");
                                    break;
                                }
                                assert!(!(layout[src].is_none() || layout[dst].is_some()), );
                                layout.swap(src, dst);
                            }
                            break;
                        }
                        space_count = 0;
                    } else {
                        space_count += 1;
                    }
                }
                #[rustfmt::skip]
                println!("{}", layout.iter().map(|opt| opt.map(|id| ((id % 10) as u8 + b'0') as char).unwrap_or('.')).collect::<String>());
                println!();
            }
            count_id = layout[layout_index];
            layout_count = 1;
        }
    }

    #[rustfmt::skip]
    println!("{}", layout.iter().map(|opt| opt.map(|id| ((id % 10) as u8 + b'0') as char).unwrap_or('.')).collect::<String>());

    let mut count: usize = 0;
    for (i, opt_id) in layout.iter().enumerate() {
        if let Some(id) = opt_id {
            count = count.checked_add(i * *id as usize).unwrap();
        }
    }
    println!("{count}");
}
