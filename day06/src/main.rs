use std::{fs, usize};

type Int = u64;

fn main() {
    let input = read_input();
    let mut field = parse_input(&input);

    let result_1 = field.walk();

    println!("Task 1: {result_1}");

    // let result_2 = todo!();
    // println!("Task 2: {result_2}");
}

fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("No input.txt found")
}
fn parse_input(input: &str) -> Field {
    let mut objects = Objects::default();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| x)
                .for_each(|x| objects.add_single(x, y))
        });
    objects.sort();

    let start = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .filter_map(|(y, line)| Some((line.chars().position(|c| c == '^')?, y)))
        .next()
        .expect("No starting position found");

    let num_rows = input.lines().filter(|line| !line.is_empty()).count();

    let num_cols = input
        .lines()
        .filter(|line| !line.is_empty())
        .next()
        .expect("No lines of input given")
        .chars()
        .count();

    let visited = vec![vec![false; num_rows]; num_cols];

    Field {
        objects,
        current: Some(start),
        visited,
        direction: Direction::Up,
    }
}

#[derive(Debug)]
struct Field {
    objects: Objects,
    current: Option<(usize, usize)>,
    visited: Vec<Vec<bool>>,
    direction: Direction,
}
impl Field {
    fn step(&mut self) {
        use Direction::*;
        match self.direction {
            Up => {
                if let Some((x, y)) = self.current {
                    let current_line = &self.objects.xs[x];
                    match current_line.binary_search(&y) {
                        Ok(_) => {
                            self.current = None;
                        }
                        Err(pos) => {
                            if pos == 0 {
                                self.visited[x][0..=y].fill(true);
                                self.current = None;
                            } else {
                                let object = current_line[pos - 1];
                                self.visited[x][object + 1..=y].fill(true);
                                self.current = Some((x, object + 1));
                                self.direction.right();
                            }
                        }
                    }
                }
            }
            Down => {
                if let Some((x, y)) = self.current {
                    let current_line = &self.objects.xs[x];
                    match current_line.binary_search(&y) {
                        Ok(_) => {
                            self.current = None;
                        }
                        Err(pos) => {
                            if pos == current_line.len() {
                                self.visited[x][y..].fill(true);
                                self.current = None;
                            } else {
                                let object = current_line[pos];
                                self.visited[x][y..object].fill(true);
                                self.current = Some((x, object - 1));
                                self.direction.right();
                            }
                        }
                    }
                }
            }
            Left => {
                if let Some((x, y)) = self.current {
                    let current_line = &self.objects.ys[y];
                    match current_line.binary_search(&x) {
                        Ok(_) => {
                            self.current = None;
                        }
                        Err(pos) => {
                            if pos == current_line.len() {
                                self.visited
                                    .iter_mut()
                                    .enumerate()
                                    .filter(|(i, _)| *i <= x)
                                    .for_each(|(_, b)| b[y] = true);
                                self.current = None;
                            } else {
                                let object = current_line[pos - 1];
                                self.visited
                                    .iter_mut()
                                    .enumerate()
                                    .filter(|(i, _)| *i <= x && *i > object)
                                    .for_each(|(_, b)| b[y] = true);
                                self.current = Some((object + 1, y));
                                self.direction.right();
                            }
                        }
                    }
                }
            }
            Right => {
                if let Some((x, y)) = self.current {
                    let current_line = &self.objects.ys[y];
                    match current_line.binary_search(&x) {
                        Ok(_) => {
                            self.current = None;
                        }
                        Err(pos) => {
                            if pos == current_line.len() {
                                self.visited
                                    .iter_mut()
                                    .enumerate()
                                    .filter(|(i, _)| *i >= x)
                                    .for_each(|(_, b)| b[y] = true);
                                self.current = None;
                            } else {
                                let object = current_line[pos];
                                self.visited
                                    .iter_mut()
                                    .enumerate()
                                    .filter(|(i, _)| *i >= x && *i < object)
                                    .for_each(|(_, b)| b[y] = true);
                                self.current = Some((object - 1, y));
                                self.direction.right();
                            }
                        }
                    }
                }
            }
        }
    }

    fn walk(&mut self) -> Int {
        while self.current.is_some() {
            self.step();
        }
        self.count()
    }
    fn count(&self) -> Int {
        self.visited
            .iter()
            .map(|line| line.iter().map(|b| *b as Int).sum::<Int>())
            .sum()
    }
}

#[derive(Debug, Default)]
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
}

#[derive(Default, Debug)]
struct Objects {
    xs: Vec<Vec<usize>>,
    ys: Vec<Vec<usize>>,
}
impl Objects {
    //  x 0 1 2 3
    // y
    // 0
    // 1
    // 2
    // 3
    fn add_single(&mut self, x: usize, y: usize) {
        if x + 1 > self.xs.len() {
            self.xs.resize_with(x + 1, Default::default);
        }
        self.xs[x].push(y);

        if y + 1 > self.ys.len() {
            self.ys.resize_with(y + 1, Default::default);
        }
        self.ys[y].push(x);
    }

    fn sort(&mut self) {
        self.xs.iter_mut().for_each(|ys| ys.sort_unstable());

        self.ys.iter_mut().for_each(|xs| xs.sort_unstable());
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

        let result = field.walk();
        field
            .visited
            .iter()
            .map(|row| {
                row.iter()
                    .map(|b| if *b { 'x' } else { '.' })
                    .collect::<String>()
            })
            .for_each(|row| eprintln!("{row}"));
        eprintln!("{:?}", field.objects.xs);
        eprintln!("{:?}", field.objects.ys);
        //         field
        //             .objects
        //             .xs
        //             .iter()
        //             .map(|row| {
        //                 let mut result = vec![','; row[row.len() - 1] + 1];
        //                 for o in row {
        //                     result[*o] = '#';
        //                 }
        //                 result.iter().collect::<String>()
        //             })
        //             .for_each(|row| eprintln!("{row}"));

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

        // let sum = todo!();

        // assert_eq!(sum, 123);
    }
}
