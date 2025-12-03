fn main() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    let mut m: Matrix<char> = Matrix::new(input.lines().map(|l| l.chars().collect()).collect());

    let mut count = 0;
    for x in 0..m.inner.len() {
        for y in 0..m.inner.len() {
            let ch = *m.get(x, y).unwrap();
            if ch != '0' {
                let mut misses = 0;
                let mut hits = 0;
                group_recursive(&mut m, ch, x, y, &mut misses, &mut hits);
                count += hits * misses;

                // Wipe current search artifacts
                m.inner.iter_mut().for_each(|l| {
                    l.iter_mut().for_each(|ch| {
                        if *ch == '1' {
                            *ch = '0'
                        }
                    })
                });
            }
        }
    }

    println!("{count}")
}

fn group_recursive(
    m: &mut Matrix<char>,
    search: char,
    x: usize,
    y: usize,
    misses: &mut usize,
    hits: &mut usize,
) {
    let i = m.get_mut(x, y).unwrap();
    if *i == search {
        *i = '1';
        *hits += 1;

        for (rx, ry) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            if let (Some(nx), Some(ny)) = (x.checked_add_signed(rx), y.checked_add_signed(ry)) {
                if nx < m.inner.len() && ny < m.inner.len() {
                    group_recursive(m, search, nx, ny, misses, hits);
                } else {
                    *misses += 1;
                }
            } else {
                *misses += 1;
            }
        }
    } else if *i != '1' {
        *misses += 1;
    }
}

#[derive(Clone)]
struct Matrix<T> {
    pub inner: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(inner: Vec<Vec<T>>) -> Self {
        let size = inner.len();
        inner.iter().for_each(|l| assert_eq!(l.len(), size));
        Self { inner }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.inner.get(x)?.get(y)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.inner.get_mut(x)?.get_mut(y)
    }
}
