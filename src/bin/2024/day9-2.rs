fn main() {
    let input = std::fs::read_to_string("input/day9.txt").unwrap();

    // [(id, pos, len)]
    let mut disk = Vec::new();
    let mut read_space = false;
    let mut index = 0;
    let mut id = 0;
    for i in input.trim().chars().map(|ch| (ch as u8 - b'0') as usize) {
        if !read_space {
            disk.push((id, index, i));
            id += 1;
        }

        index += i;
        read_space = !read_space;
    }

    debug_print(&disk);

    'id: for id in (1..id).rev() {
        let file_pos = disk.iter().position(|(fid, _, _)| *fid == id).unwrap();
        let (id, pos, len) = disk[file_pos];
        //debug_print(&disk);

        let mut lf_end = 0;
        for (i, &(fid, fpos, flen)) in disk.iter().enumerate() {
            if fpos - lf_end >= len {
                disk.remove(file_pos);
                disk.insert(i, (id, lf_end, len));
                continue 'id;
            }
            if fid == id {
                continue 'id;
            }
            lf_end = fpos + flen;
        }
        disk.remove(file_pos);
        disk.push((id, pos, len));
    }

    if !disk.is_sorted_by_key(|(_, pos, _)| pos) {
        unreachable!("Disk not sorted");
    }

    debug_print(&disk);

    let mut count: usize = 0;
    for &(id, pos, len) in disk.iter() {
        for i in pos..pos + len {
            println!("{i} * {id} = {}", i * id);
            count = count.checked_add(i * id).unwrap();
        }
    }
    println!("{count}");
}

fn debug_print(disk: &Vec<(usize, usize, usize)>) {
    let mut index = 0;
    for &(id, pos, len) in disk {
        for _ in index..pos {
            print!(".");
        }
        for _ in 0..len {
            print!("{}", id % 10);
        }
        index = pos + len;
    }
    println!();
}
