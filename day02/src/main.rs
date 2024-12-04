use std::fs;

type Int = u64;

fn main() {
    let input = read_input();
    let sum = num_safe_reports(&input);

    println!("Task 1: {sum}");

    let sum = num_safe_dampened_reports(&input);

    println!("Task 2: {sum}");
}

fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("No input.txt found")
}

fn num_safe_reports(input: &str) -> Int {
    input
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<Int>().ok())
                .collect::<Option<Vec<_>>>()
        })
        .map(|values| is_ascending(&values) == Ok(()) || is_descending(&values) == Ok(()))
        .map(|is_safe| is_safe as Int)
        .sum()
}

fn is_ascending(report: &[Int]) -> Result<(), usize> {
    let mut iter = report.iter().enumerate();
    let Some((_, mut last_value)) = iter.next() else {
        return Err(0);
    };

    while let Some((i, value)) = iter.next() {
        if value <= last_value || *value > last_value + 3 {
            return Err(i);
        }
        last_value = value;
    }
    return Ok(());
}

fn is_descending(report: &[Int]) -> Result<(), usize> {
    let mut iter = report.iter().enumerate();
    let Some((_, mut last_value)) = iter.next() else {
        return Err(0);
    };

    while let Some((i, value)) = iter.next() {
        if value >= last_value || value + 3 < *last_value {
            return Err(i);
        }
        last_value = value;
    }
    return Ok(());
}

fn num_safe_dampened_reports(input: &str) -> Int {
    input
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<Int>().ok())
                .collect::<Option<Vec<_>>>()
        })
        .map(|values| is_ascending_dampened(&values) || is_descending_dampened(&values))
        .map(|is_safe| is_safe as Int)
        .sum()
}

fn is_ascending_dampened(report: &[Int]) -> bool {
    match is_ascending(report) {
        Ok(()) => true,
        Err(n) if n > 0 => {
            let mut left_pop = report.to_owned();
            left_pop.remove(n - 1);

            let mut right_pop = report.to_owned();
            right_pop.remove(n);

            is_ascending(&left_pop) == Ok(()) || is_ascending(&right_pop) == Ok(())
        }
        Err(_) => false,
    }
}

fn is_descending_dampened(report: &[Int]) -> bool {
    match is_descending(report) {
        Ok(()) => true,
        Err(n) if n > 0 => {
            let mut left_pop = report.to_owned();
            left_pop.remove(n - 1);

            let mut right_pop = report.to_owned();
            right_pop.remove(n);

            is_descending(&left_pop) == Ok(()) || is_descending(&right_pop) == Ok(())
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        let test_input = "7 6 4 2 1\n\
                                1 2 7 8 9\n\
                                9 7 6 2 1\n\
                                1 3 2 4 5\n\
                                8 6 4 4 1\n\
                                1 3 6 7 9\n";
        let sum = num_safe_reports(test_input);
        assert_eq!(sum, 2);
    }

    #[test]
    fn task2() {
        let test_input = "7 6 4 2 1\n\
                                1 2 7 8 9\n\
                                9 7 6 2 1\n\
                                1 3 2 4 5\n\
                                8 6 4 4 1\n\
                                1 3 6 7 9\n";
        let sum = num_safe_dampened_reports(test_input);
        assert_eq!(sum, 4);
    }
}
