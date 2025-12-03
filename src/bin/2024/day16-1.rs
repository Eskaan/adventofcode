use std::io::{self, Read};

fn main() {
    let input = std::fs::read_to_string("input/day16.txt").unwrap();
    let mut map: Matrix<Tile> = Matrix::new(
        input
            .lines()
            .map(|l| l.chars().map(|ch| ch.into()).collect())
            .collect(),
    );
    let end = (1, map.inner.len() - 2);
    let start = (map.inner.len() - 2, 1);

    //map.routing_fill(start, 1, 0);
    map.dijkstra(start);
    map.debug_print();

    println!("Finish: {:?}", map.get(end).unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Score(usize, usize),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' | 'S' | 'E' => Self::Open,
            '#' => Self::Wall,
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
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn routing_fill(&mut self, tile: (usize, usize), dir: usize, score: usize) {
        //println!("{tile:?} - {dir:?}");
        *self.get_mut(tile).unwrap() = Tile::Score(score, 0);
        for i in [3, 0, 1] {
            let dscore = score
                + match i {
                    3 => 1001,
                    0 => 1,
                    1 => 1001,
                    _ => unreachable!(),
                };
            let dir_index = (dir + i) % 4;
            //println!("{dir} + {i} => {dir_index} {}", dir + i);
            let ddir = Self::DIRECTIONS[dir_index];

            let next_tile = (
                tile.0.checked_add_signed(ddir.0).unwrap(),
                tile.1.checked_add_signed(ddir.1).unwrap(),
            );
            match *self.get(next_tile).unwrap() {
                Tile::Wall => {}
                Tile::Open => {
                    self.routing_fill(next_tile, dir_index, dscore);
                }
                Tile::Score(next_score, _) => {
                    if score < next_score {
                        self.routing_fill(next_tile, dir_index, dscore);
                    }
                }
            }
        }
    }

    fn dijkstra(&mut self, from: (usize, usize)) {
        let mut buf = Vec::new();
        let mut score = 0;
        buf.push((from, 1, 0));
        while !buf.is_empty() {
            //while score < 5000 {
            if let Some(curr) = buf.iter().position(|(_, _, i)| *i == score) {
                let (tile, dir, _) = buf.remove(curr);
                //self.debug_print();
                //io::stdin().read_line(&mut String::new()).unwrap();
                println!("Running {score} {tile:?} {dir}");

                *self.get_mut(tile).unwrap() = Tile::Score(score, dir);
                for i in [3, 0, 1, 2] {
                    let next_score = score
                        + match i {
                            3 => 1001,
                            0 => 1,
                            1 => 1001,
                            2 => 2001,
                            _ => unreachable!(),
                        };
                    let next_dir_index = (dir + i) % 4;
                    let next_dir = Self::DIRECTIONS[next_dir_index];
                    let next_tile = (
                        tile.0.checked_add_signed(next_dir.0).unwrap(),
                        tile.1.checked_add_signed(next_dir.1).unwrap(),
                    );
                    //println!("{dir} + {i} => {dir_index} {}", dir + i);

                    match *self.get(next_tile).unwrap() {
                        Tile::Wall => {}
                        Tile::Open => {
                            buf.push((next_tile, next_dir_index, next_score));
                        }
                        Tile::Score(tile_score, tile_dir) => {
                            if tile_dir == next_dir_index && next_score < tile_score {
                                buf.push((next_tile, next_dir_index, next_score));
                            }
                        }
                    }
                }
            } else {
                score += 1;
            }
        }
    }

    fn debug_print(&self) {
        for line in &self.inner {
            for t in line {
                match t {
                    Tile::Open => print!(" ++ "),
                    Tile::Wall => print!("    "),
                    Tile::Score(s, _) => print!("{:0>4}", s % 10000),
                }
            }
            println!()
        }
    }
}
