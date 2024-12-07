fn main() {
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    // Note that the matrix is read in with (y, x)
    let m = Matrix::new(input.clone());

    let start_index = input
        .chars()
        .filter(|ch| *ch != '\n')
        .enumerate()
        .find(|(_, ch)| *ch == '^')
        .unwrap()
        .0;

    // (pos, direction)
    let start_pos = Pos(
        (start_index / m.size) as isize,
        (start_index % m.size) as isize,
    );
    let mut path_pos = start_pos;
    let mut path_dir = Pos(-1, 0);

    let mut path = 0;
    while let Some(next) = m.get(path_pos) {
        if next {
            // Step back and turn
            path -= 1;
            path_pos = path_pos - path_dir;
            path_dir = path_dir.turn();
        } else {
            path += 1;
            path_pos = path_pos + path_dir;
        }
    }

    println!("{path}");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(isize, isize);

impl Pos {
    pub fn turn(&self) -> Self {
        match self {
            Pos(0, -1) => Pos(-1, 0),
            Pos(-1, 0) => Pos(0, 1),
            Pos(0, 1) => Pos(1, 0),
            Pos(1, 0) => Pos(0, -1),
            _ => panic!("invalid arg"),
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct Matrix {
    pub size: usize,
    pub inner: Vec<Vec<bool>>,
}

impl Matrix {
    pub fn new(input: String) -> Self {
        let size = input.lines().count();
        input.lines().for_each(|l| assert_eq!(l.len(), size));

        Self {
            size: size,
            inner: input
                .lines()
                .map(|l| l.chars().map(|ch| ch == '#').collect())
                .collect(),
        }
    }

    pub fn get(&self, pos: Pos) -> Option<bool> {
        if pos.0 < 0 || pos.1 < 0 {
            None
        } else {
            self.inner.get(pos.0 as usize)?.get(pos.1 as usize).copied()
        }
    }
}
