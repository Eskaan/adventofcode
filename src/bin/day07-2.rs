use std::collections::HashMap;

fn main() {
    // Parse inputs
    let lines = std::fs::read_to_string("input/2025/day7.txt")
        .unwrap()
        .trim_end()
        .lines()
        .map(str::to_string)
        .collect::<Vec<_>>();

    let mut beams = HashMap::new();
    beams.insert(lines[0].find('S').unwrap(), 1usize);

    for line in &lines[1..] {
        let mut keys = beams.clone().into_keys().collect::<Vec<_>>();
        keys.sort();
        for cur_beam in keys {
            if line.as_bytes()[cur_beam] == b'^' {
                let cur_timelines = beams.remove(&cur_beam).unwrap();
                if let Some(timelines) = beams.get_mut(&(cur_beam - 1)) {
                    *timelines += cur_timelines;
                } else {
                    beams.insert(cur_beam - 1, cur_timelines);
                }

                if let Some(timelines) = beams.get_mut(&(cur_beam + 1)) {
                    *timelines += cur_timelines;
                } else {
                    beams.insert(cur_beam + 1, cur_timelines);
                }
            }
        }

        println!("{beams:#?}");
    }

    let counter = beams.into_values().fold(0, |f, i| f + i);
    println!("counter: {counter}");
}
