fn main() {
    let input = std::fs::read_to_string("input/day15.txt").unwrap();
    let (map, moves) = input.split_once("\n\n").unwrap();
    let moves: Vec<char> = moves.replace('\n', "").chars().collect();
    let mut robot = map
        .lines()
        .enumerate()
        .find_map(|(x, l)| Some((x, l.chars().enumerate().find(|(_, ch)| *ch == '@')?.0)))
        .unwrap();
    let mut map: Matrix<Tile> = Matrix::new(
        map.lines()
            .map(|l| {
                l.chars()
                    .map(|ch| ch.into())
                    .collect()
            })
            .collect(),
    );

    for m in moves {
        let dir = match m {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!("Parse err: Invalid char '{m}' for Direction"),
        };
        if map.try_move(robot, dir) {
            robot = (
                robot.0.checked_add_signed(dir.0).unwrap(),
                robot.1.checked_add_signed(dir.1).unwrap(),
            );
        }
    }

    println!("{}", map.gps_score())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Box,
    Robot,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Open,
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            _ => panic!("Parse err: Invalid char '{value}' for Tile"),
        }
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

    pub fn get(&self, pos: (usize, usize)) -> Option<&T> {
        self.inner.get(pos.0)?.get(pos.1)
    }
    pub fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut T> {
        self.inner.get_mut(pos.0)?.get_mut(pos.1)
    }
}

impl Matrix<Tile> {
    fn try_move(&mut self, tile: (usize, usize), dir: (isize, isize)) -> bool {
        let next_tile = (
            tile.0.checked_add_signed(dir.0).unwrap(),
            tile.1.checked_add_signed(dir.1).unwrap(),
        );
        match *self.get(next_tile).unwrap() {
            Tile::Open => {
                *self.get_mut(next_tile).unwrap() = *self.get(tile).unwrap();
                *self.get_mut(tile).unwrap() = Tile::Open;
                true
            }
            Tile::Wall => false,
            Tile::Box => {
                if self.try_move(next_tile, dir) {
                    *self.get_mut(next_tile).unwrap() = *self.get(tile).unwrap();
                    *self.get_mut(tile).unwrap() = Tile::Open;
                    true
                } else {
                    false
                }
            }
            Tile::Robot => unreachable!(),
        }
    }

    fn gps_score(&self) -> usize {
        let mut score = 0;

        for x in 0..self.inner.len() {
            for y in 0..self.inner.len() {
                if *self.get((x, y)).unwrap() == Tile::Box {
                    score += (100 * x) + y;
                }
            }
        }

        score
    }
}
