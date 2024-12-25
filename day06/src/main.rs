use std::{fs, usize};

type Int = u64;

fn main() {
    let input = read_input();
    let mut field = parse_input(&input);

    field.walk();
    let result_1 = field.count_walked();

    println!("Task 1: {result_1}");

    let result_2 = count_new_objects(&input);
    println!("Task 2: {result_2}");
}

fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("No input.txt found")
}
fn parse_input(input: &str) -> Field {
    let start = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .filter_map(|(y, line)| Some((line.chars().position(|c| c == '^')?, y)))
        .next()
        .expect("No starting position found");

    let tiles = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Tile>>())
        .collect();

    Field {
        position: Position::Coordinates(start.0, start.1),
        tiles,
        direction: Direction::Up,
    }
}

fn count_new_objects(input: &str) -> Int {
    let field = parse_input(input);
    field
        .tiles
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| matches!(tile, Tile::None))
                .map(|(x, _)| {
                    let mut field_copy = field.clone();
                    field_copy.tiles[y][x] = Tile::Object;
                    field_copy.walk();
                    matches!(field_copy.position, Position::InCycle) as Int
                })
                .sum::<Int>()
        })
        .sum::<Int>()
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Object,
    Start { directions: [bool; 4] },
    Walked { directions: [bool; 4] },
    None,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Start {
                directions: [true, false, false, false],
            },
            '#' => Self::Object,
            _ => Self::None,
        }
    }
}
impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Self::Object => '#',
            Self::Start { .. } => '^',
            Self::Walked { .. } => 'o',
            Self::None { .. } => '.',
        }
    }
}

//  x 0 1 2 3
// y
// 0
// 1
// 2
// 3

#[derive(Debug, Clone, Copy)]
enum Position {
    Coordinates(usize, usize),
    OutOfBounds,
    InCycle,
}

#[derive(Debug, Clone)]
struct Field {
    position: Position,
    tiles: Vec<Vec<Tile>>,
    direction: Direction,
}
impl Field {
    fn step(&mut self) {
        use Direction::*;
        use Position::*;
        use Tile::*;

        let Coordinates(x, y) = self.position else {
            return;
        };

        let next_pos = match self.direction {
            Down => (x as isize, y as isize + 1),
            Up => (x as isize, y as isize - 1),
            Right => (x as isize + 1, y as isize),
            Left => (x as isize - 1, y as isize),
        };

        // check lower bounds
        if next_pos.0 < 0 || next_pos.1 < 0 {
            self.position = OutOfBounds;
            return;
        }
        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

        // step
        if let Some(Some(tile)) = self.tiles.get(next_pos.1).map(|row| row.get(next_pos.0)) {
            match tile {
                None => {
                    self.position = Coordinates(next_pos.0, next_pos.1);
                    let mut directions = [false; 4];
                    directions[self.direction.ordinal()] = true;
                    self.tiles[next_pos.1][next_pos.0] = Walked { directions };
                }
                Start { mut directions } | Walked { mut directions, .. } => {
                    if directions[self.direction.ordinal()] {
                        self.position = Position::InCycle;
                        return;
                    }
                    directions[self.direction.ordinal()] = true;
                    self.position = Coordinates(next_pos.0, next_pos.1);
                }
                Object => {
                    self.direction.right();
                }
            }
        } else {
            self.position = OutOfBounds;
        }
    }

    fn walk(&mut self) {
        while matches!(self.position, Position::Coordinates(_, _)) {
            self.step();
        }
    }

    fn count_walked(&self) -> Int {
        self.tiles
            .iter()
            .map(|line| {
                line.iter()
                    .map(|tile| match tile {
                        Tile::Start { .. } | Tile::Walked { .. } => 1,
                        _ => 0,
                    })
                    .sum::<Int>()
            })
            .sum()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn right(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Right,
            Down => Left,
            Right => Down,
            Left => Up,
        };
    }
    fn ordinal(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        let test_input = "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...\n\
            ";

        let mut field = parse_input(&test_input);

        field.walk();
        let result = field.count_walked();

        assert_eq!(result, 41);
    }

    #[test]
    fn task2() {
        let test_input = "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...\n\
            ";

        let mut field = parse_input(&test_input);

        field.walk();
        let result = count_new_objects(&test_input);

        assert_eq!(result, 6);
    }
}
