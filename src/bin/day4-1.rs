use std::fs;

// Relative neighbors in grid
#[rustfmt::skip]
const SEARCH: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1),
];

fn main() {
    let m = Matrix::new(&fs::read_to_string("input/day4.txt").unwrap());
    let mut count = 0;

    for x in 0..m.size {
        for y in 0..m.size {
            'search: for (xr, yr) in SEARCH {
                for (i, ch) in "XMAS".chars().enumerate() {
                    if m.get_relative(x, y, xr * i as isize, yr * i as isize) != Some(ch) {
                        continue 'search;
                    }
                }
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
    pub fn new(input: &str) -> Self {
        let size = input.lines().count();
        input.lines().for_each(|l| assert_eq!(l.len(), size));

        Self {
            size,
            inner: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    /*pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.inner.get(x)?.get(y).copied()
    }*/

    pub fn get_relative(&self, x: usize, y: usize, xr: isize, yr: isize) -> Option<char> {
        self.inner
            .get(x.checked_add_signed(xr)?)?
            .get(y.checked_add_signed(yr)?)
            .copied()
    }
}
