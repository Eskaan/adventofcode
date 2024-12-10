fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    // Note that the matrix is read in with (y, x)
    let m = Matrix::new(&input);

    let mut count = 0;
    for x in 0..m.inner.len() as isize {
        for y in 0..m.inner.len() as isize {
            if m.get(x, y).unwrap() == 0 {
                let mut peaks = Vec::new();
                paths_recursive(&m, 0, x, y, &mut peaks);
                count += peaks.len();
            }
        }
    }
    println!("{count}");
}

fn paths_recursive(m: &Matrix, curr: u8, x: isize, y: isize, peaks: &mut Vec<(isize, isize)>) {
    for neigh in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let nx = x + neigh.0;
        let ny = y + neigh.1;
        if m.get(nx, ny) == Some(curr + 1) {
            if curr >= 8 {
                peaks.push((nx, ny));
                continue;
            }
            paths_recursive(m, curr + 1, nx, ny, peaks);
        }
    }
}

struct Matrix {
    pub inner: Vec<Vec<u8>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let size = input.lines().count();
        input.lines().for_each(|l| assert_eq!(l.len(), size));

        Self {
            inner: input
                .lines()
                .map(|l| l.chars().map(|ch| ch as u8 - b'0').collect())
                .collect(),
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || y < 0 {
            None
        } else {
            self.inner.get(x as usize)?.get(y as usize).copied()
        }
    }
}
