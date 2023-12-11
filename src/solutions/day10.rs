use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn offset(&self) -> Position {
        match self {
            Direction::North => Position::new(0, -1),
            Direction::East => Position::new(1, 0),
            Direction::South => Position::new(0, 1),
            Direction::West => Position::new(-1, 0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Ground,
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Tile {
    fn from_byte(b: &u8) -> Self {
        use Tile::*;
        match b {
            b'|' => Vertical,
            b'-' => Horizontal,
            b'L' => NorthEast,
            b'J' => NorthWest,
            b'7' => SouthWest,
            b'F' => SouthEast,
            b'.' => Ground,
            b'S' => Start,
            _ => panic!(),
        }
    }

    fn next_direction(&self, from: Direction) -> Direction {
        use Direction::*;
        use Tile::*;
        match from {
            North => match self {
                Vertical => North,
                SouthWest => West,
                SouthEast => East,
                _ => panic!(),
            },
            East => match self {
                Horizontal => East,
                NorthWest => North,
                SouthWest => South,
                _ => panic!(),
            },
            South => match self {
                Vertical => South,
                NorthEast => East,
                NorthWest => West,
                _ => panic!(),
            },
            West => match self {
                Horizontal => West,
                NorthEast => North,
                SouthEast => South,
                _ => panic!(),
            },
        }
    }
}

#[derive(Clone, PartialEq)]
enum Fill {
    Empty,
    Pipe,
}

struct Grid {
    path: Vec<(Position, Direction)>,
    area: usize,
}

impl Grid {
    fn parse(s: String) -> Self {
        let mut tiles = Vec::new();
        let mut fill_grid = Vec::new();

        s.lines().for_each(|line| {
            tiles.push(
                line.as_bytes()
                    .iter()
                    .map(Tile::from_byte)
                    .collect::<Vec<Tile>>(),
            );
            fill_grid.push(vec![Fill::Empty; tiles[0].len()]);
        });

        let mut start = None;
        for (y, row) in tiles.iter_mut().enumerate() {
            for (x, tile) in row.iter_mut().enumerate() {
                if *tile == Tile::Start {
                    *tile = Tile::SouthEast; // TODO: dynamically solve what the start tile should be
                    start = Some(Position::new(x as isize, y as isize))
                }
            }
        }
        let start = start.unwrap();

        let mut current = start;
        let mut path = Vec::new();
        let mut dir = Direction::North;
        path.push((current, Direction::East));

        fill_grid[start.y as usize][start.x as usize] = Fill::Pipe;

        loop {
            let tile = tiles[current.y as usize][current.x as usize];
            dir = tile.next_direction(dir);
            current += dir.offset();
            if current == start {
                break;
            }

            fill_grid[current.y as usize][current.x as usize] = Fill::Pipe;

            path.push((current, dir));
        }

        #[derive(Debug)]
        enum ScanState {
            Out,
            OutTop,
            OutBot,
            In,
            InTop,
            InBot,
        }

        use ScanState::*;
        let mut area = 0;
        for (y, row) in fill_grid.iter().enumerate() {
            let mut state = Out;
            for (x, f) in row.iter().enumerate() {
                if *f == Fill::Pipe {
                    match tiles[y][x] {
                        Tile::SouthEast => match state {
                            Out => state = InTop,
                            In => state = OutTop,
                            _ => panic!(),
                        },
                        Tile::NorthEast => match state {
                            Out => state = InBot,
                            In => state = OutBot,
                            _ => panic!(),
                        },
                        Tile::SouthWest => match state {
                            InTop => state = Out,
                            InBot => state = In,
                            OutBot => state = Out,
                            OutTop => state = In,
                            _ => panic!(),
                        },
                        Tile::NorthWest => match state {
                            InTop => state = In,
                            InBot => state = Out,
                            OutBot => state = In,
                            OutTop => state = Out,
                            _ => panic!(),
                        },
                        Tile::Vertical => match state {
                            Out => state = In,
                            In => state = Out,
                            _ => panic!(),
                        },
                        _ => (),
                    }
                } else {
                    match state {
                        In => {
                            area += 1;
                        }
                        _ => (),
                    }
                }
            }
        }

        return Grid { path, area };
    }
}

pub fn run(input: String) -> Option<(String, String)> {
    let grid = Grid::parse(input);

    let part1 = grid.path.len() / 2;

    let part2 = grid.area;

    Some((part1.to_string(), part2.to_string()))
}
