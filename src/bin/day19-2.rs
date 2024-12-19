fn main() {
    let input = std::fs::read_to_string("input/day19.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut patterns: Vec<&str> = lines[0].split(", ").collect();

    // Performance hack, probably related to CPU cache misses.
    patterns.sort_unstable();
    patterns.reverse();

    let mut counter = 0;
    for design in &lines[2..] {
        let mut map = std::collections::BTreeMap::new();
        map.insert(0, 1usize);
        let patterns: Vec<&str> = patterns
            .iter()
            .filter(|&pat| design.contains(pat))
            .map(|pat| *pat)
            .collect();
        for i in 0..design.len() {
            if let Some(&options) = map.get(&i) {
                for pat in &patterns {
                    if design[i..].starts_with(pat) {
                        if let Some(value) = map.get_mut(&(i + pat.len())) {
                            *value += options;
                        } else {
                            map.insert(i + pat.len(), options);
                        }
                    }
                }
            }
        }

        if let Some(options) = map.get(&design.len()) {
            counter += options;
        }
    }
    println!("{counter}");
}
