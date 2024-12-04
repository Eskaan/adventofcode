use std::fs;

// Relative neighbors in grid
const SEARCH: [(isize, isize); 4] = [(-1, 1), (1, -1), (1, 1), (-1, -1)];

fn main() {
    let m = Matrix::new(fs::read_to_string("input/day4.txt").unwrap());
    let mut count = 0;

    for x in 0..m.size {
        for y in 0..m.size {
            if m.get(x, y).unwrap() != 'A' {
                continue;
            }
            let mut matches = 0;
            for (xr, yr) in SEARCH {
                if m.get_relative(x, y, xr, yr) == Some('M')
                    && m.get_relative(x, y, xr * -1, yr * -1) == Some('S')
                {
                    matches += 1;
                }
            }
            if matches == 2 {
                count += 1;
            }
        }
    }

    println!("{count}");
}

struct Matrix {
    pub size: usize,
    inner: Vec<Vec<char>>,
}

impl Matrix {
    pub fn new(input: String) -> Self {
        let size = input.lines().count();
        input.lines().for_each(|l| assert_eq!(l.len(), size));

        Self {
            size: size,
            inner: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.inner.get(x)?.get(y).copied()
    }

    pub fn get_relative(&self, x: usize, y: usize, xr: isize, yr: isize) -> Option<char> {
        self.inner
            .get(x.checked_add_signed(xr)?)?
            .get(y.checked_add_signed(yr)?)
            .copied()
    }
}
