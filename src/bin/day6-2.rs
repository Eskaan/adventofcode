fn main() {
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    // Note that the matrix is read in with (y, x)
    let mut m = Matrix::new(&input);

    // Find start line and char
    let start_index = input
        .chars()
        .filter(|ch| *ch != '\n')
        .enumerate()
        .find(|(_, ch)| *ch == '^')
        .unwrap()
        .0;
    let mut block: Vec<Pos> = Vec::new();

    // (pos, direction)
    let start_pos = Pos(
        (start_index / m.size) as isize,
        (start_index % m.size) as isize,
    );
    let mut path_pos = start_pos;
    let mut path_dir = Pos(-1, 0);

    let mut path: Vec<Pos> = Vec::new();
    while let Some(next) = m.get(path_pos) {
        if next {
            // Step back and turn
            path.pop();
            path_pos = path_pos - path_dir;
            path_dir = path_dir.turn();
        } else {
            path.push(path_pos);
            path_pos = path_pos + path_dir;
            // If we intersect ourselves we would've encountered this block earlier
            if !path.contains(&path_pos) {
                let mut check_dir = path_dir.turn();
                let mut check_pos = path_pos - path_dir;
                let mut check_turns = Vec::new();
                let block_was = m.get(path_pos).unwrap_or_default();
                m.set(path_pos, true);
                while let Some(next) = m.get(check_pos) {
                    if next {
                        check_pos = check_pos - check_dir;
                        check_dir = check_dir.turn();
                        if check_turns.contains(&(check_pos, check_dir)) {
                            block.push(path_pos);
                            break;
                        }
                        check_turns.push((check_pos, check_dir));
                    } else {
                        check_pos = check_pos + check_dir;
                    }
                }
                m.set(path_pos, block_was);
            }
        }
    }

    //block.sort();
    //block.dedup();

    println!("{}", block.len());
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
    pub fn new(input: &str) -> Self {
        let size = input.lines().count();
        input.lines().for_each(|l| assert_eq!(l.len(), size));

        Self {
            size,
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

    pub fn set(&mut self, pos: Pos, value: bool) {
        if let Some(line) = self.inner.get_mut(pos.0 as usize) {
            if let Some(index) = line.get_mut(pos.1 as usize) {
                *index = value;
            }
        };
    }
}
