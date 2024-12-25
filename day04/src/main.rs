use std::{fs, iter::zip};

type Int = u64;

fn main() {
    let input = read_input();

    let sum = check_all(&input);
    println!("Task 1: {sum}");

    let sum = check_all_x(&input);
    println!("Task 2: {sum}");
}

fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("No input.txt found")
}

fn check_all(input: &str) -> Int {
    let field = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_uppercase().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    field
        .clone()
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, _)| check_pos(field.clone(), (x, y)))
                .sum::<Int>()
        })
        .sum::<Int>()
}

fn check_pos(field: Vec<Vec<char>>, pos: (usize, usize)) -> Int {
    let to_search = "XMAS";
    check_up_down(&field, pos, to_search) as Int
        + check_left_right(&field, pos, to_search) as Int
        + check_down_right_up_left(&field, pos, to_search) as Int
        + check_down_left(&field, pos, to_search) as Int
        + check_up_right(&field, pos, to_search) as Int
}

// x y 0 1 2
// 0 < . . . >
// 1 < . . . >

fn check_up_down(field: &Vec<Vec<char>>, pos: (usize, usize), to_search: &str) -> Int {
    check_pattern(
        field,
        zip(to_search.chars(), zip(pos.0.., std::iter::repeat(pos.1))),
    ) as Int
        + check_pattern(
            field,
            zip(
                to_search.chars().rev(),
                zip(pos.0.., std::iter::repeat(pos.1)),
            ),
        ) as Int
}

fn check_left_right(field: &Vec<Vec<char>>, pos: (usize, usize), to_search: &str) -> Int {
    check_pattern(
        field,
        zip(to_search.chars(), zip(std::iter::repeat(pos.0), pos.1..)),
    ) as Int
        + check_pattern(
            field,
            zip(
                to_search.chars().rev(),
                zip(std::iter::repeat(pos.0), pos.1..),
            ),
        ) as Int
}

fn check_down_right_up_left(field: &Vec<Vec<char>>, pos: (usize, usize), to_search: &str) -> Int {
    check_pattern(field, zip(to_search.chars(), zip(pos.0.., pos.1..))) as Int
        + check_pattern(field, zip(to_search.chars().rev(), zip(pos.0.., pos.1..))) as Int
}

fn check_down_left(field: &Vec<Vec<char>>, pos: (usize, usize), to_search: &str) -> bool {
    check_pattern(
        field,
        zip(
            to_search.chars(),
            zip(pos.0.., (pos.1..(pos.1 + to_search.len())).rev()),
        ),
    )
}

fn check_up_right(field: &Vec<Vec<char>>, pos: (usize, usize), to_search: &str) -> bool {
    if pos.0 + 1 < to_search.len() {
        return false;
    }
    check_pattern(
        field,
        zip(
            to_search.chars(),
            zip((pos.0 + 1 - to_search.len()..=pos.0).rev(), pos.1..),
        ),
    )
}

fn check_pattern<I>(field: &Vec<Vec<char>>, pattern: I) -> bool
where
    I: IntoIterator<Item = (char, (usize, usize))>,
{
    pattern
        .into_iter()
        .map(|(value, (x, y))| field.get(x).map(|row| row.get(y)).flatten() == Some(&value))
        .all(|v| v)
}

fn check_all_x(input: &str) -> Int {
    let field = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_uppercase().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    field
        .clone()
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, _)| check_x_pos(&field, (x, y)))
                .sum::<Int>()
        })
        .sum::<Int>()
}

fn check_x_pos(field: &Vec<Vec<char>>, pos: (usize, usize)) -> Int {
    let to_search = "MASMAS";

    // M M
    //  A
    // S S
    let result_0 = check_pattern(
        field,
        zip(
            to_search.chars(),
            vec![
                (pos.0 + 0, pos.1 + 0),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 2, pos.1 + 2),
                (pos.0 + 0, pos.1 + 2),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 2, pos.1 + 0),
            ]
            .into_iter(),
        ),
    );
    // M S
    //  A
    // M s
    let result_1 = check_pattern(
        field,
        zip(
            to_search.chars(),
            vec![
                (pos.0 + 0, pos.1 + 0),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 2, pos.1 + 2),
                (pos.0 + 2, pos.1 + 0),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 0, pos.1 + 2),
            ]
            .into_iter(),
        ),
    );
    // S S
    //  A
    // M M
    let result_2 = check_pattern(
        field,
        zip(
            to_search.chars(),
            vec![
                (pos.0 + 2, pos.1 + 0),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 0, pos.1 + 2),
                (pos.0 + 2, pos.1 + 2),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 0, pos.1 + 0),
            ]
            .into_iter(),
        ),
    );
    // S M
    //  A
    // S M
    let result_3 = check_pattern(
        field,
        zip(
            to_search.chars(),
            vec![
                (pos.0 + 2, pos.1 + 2),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 0, pos.1 + 0),
                (pos.0 + 0, pos.1 + 2),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 2, pos.1 + 0),
            ]
            .into_iter(),
        ),
    );

    result_0 as Int + result_1 as Int + result_2 as Int + result_3 as Int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task0() {
        let test_input = "\
            ..X...\n\
            .SAMX.\n\
            .A..A.\n\
            XMAS.S\n\
            .X....\n\
            ";
        let sum = check_all(test_input);
        assert_eq!(sum, 4);
    }

    #[test]
    fn task1() {
        let test_input = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX\n\
            ";
        let sum = check_all(test_input);
        assert_eq!(sum, 18);
    }

    #[test]
    fn task2() {
        let test_input = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX\n\
            ";
        let sum = check_all_x(test_input);
        assert_eq!(sum, 9);
    }
}
