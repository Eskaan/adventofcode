fn main() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    let mut m: Matrix<char> = Matrix::new(input.lines().map(|l| l.chars().collect()).collect());

    let mut count = 0;
    for x in 0..m.inner.len() {
        for y in 0..m.inner.len() {
            let ch = *m.get(x, y).unwrap();
            if ch != '0' {
                let mut misses = Vec::new();
                let mut hits = 0;
                group_recursive(&mut m, ch, x, y, &mut misses, &mut hits);

                let mut sides = 0;
                while let Some((x, y, rx, ry)) = misses.pop() {
                    sides += 1;

                    let mut nx = x;
                    let mut ny = y;
                    let mut nrx = rx;
                    let mut nry = ry;
                    // Follow point in both directions
                    for _ in 0..2 {
                        loop {
                            nx = match nx.checked_add_signed(nry) {
                                Some(i) => i,
                                None => break,
                            };
                            ny = match ny.checked_add_signed(nrx) {
                                Some(i) => i,
                                None => break,
                            };
                            if let Some(index) =
                                misses.iter().position(|pos| *pos == (nx, ny, rx, ry))
                            {
                                misses.remove(index);
                            } else {
                                break;
                            }
                        }
                        nx = x;
                        ny = y;
                        nrx = -nrx;
                        nry = -nry;
                    }
                }

                count += hits * sides;

                // Wipe current search artifacts
                // '0' is reserved as a marking char for a matched field
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

/// '1' is reserved as a marking char for matched items
fn group_recursive(
    m: &mut Matrix<char>,
    search: char,
    x: usize,
    y: usize,
    misses: &mut Vec<(usize, usize, isize, isize)>,
    hits: &mut usize,
) {
    let i = m.get_mut(x, y).unwrap();
    if *i == search {
        *i = '1';
        *hits += 1;

        for (rx, ry) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            if let (Some(nx), Some(ny)) = (x.checked_add_signed(rx), y.checked_add_signed(ry)) {
                if nx < m.inner.len() && ny < m.inner.len() {
                    let nch = *m.get(nx, ny).unwrap();
                    if nch == search {
                        group_recursive(m, search, nx, ny, misses, hits);
                    } else if nch != '1' {
                        misses.push((x, y, rx, ry));
                    }
                } else {
                    misses.push((x, y, rx, ry));
                }
            } else {
                misses.push((x, y, rx, ry));
            }
        }
    } else {
        unreachable!()
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
