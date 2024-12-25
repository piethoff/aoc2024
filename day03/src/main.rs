use std::fs;

type Int = u64;

fn main() {
    let input = read_input();

    let sum = parse(&input).iter().sum::<Int>();
    println!("Task 1: {sum}");

    let sum = parse_(&input).iter().sum::<Int>();
    println!("Task 2: {sum}");
}

fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("No input.txt found")
}

fn parse(mut input: &str) -> Vec<Int> {
    let mut result = vec![];
    while !input.is_empty() {
        match parse_mul(&mut input) {
            Some((rest, (a, b))) => {
                result.push(a * b);
                input = rest;
            }
            None => input = &input[1..],
        }
    }

    result
}

fn parse_(mut input: &str) -> Vec<Int> {
    let mut result = vec![];
    let mut doing = true;

    while !input.is_empty() {
        match parse_do(input) {
            Some((rest, new_do)) => {
                doing = new_do;
                input = rest;
                continue;
            }
            None => {}
        }
        if doing {
            match parse_mul(input) {
                Some((rest, (a, b))) => {
                    result.push(a * b);
                    input = rest;
                    continue;
                }
                None => {}
            }
        }
        input = &input[1..];
    }

    result
}

fn parse_do(input: &str) -> Option<(&str, bool)> {
    if input.starts_with("don't()") {
        return Some((&input[7..], false));
    }
    if input.starts_with("do()") {
        return Some((&input[4..], true));
    }
    None
}

fn parse_mul(input: &str) -> Option<(&str, (Int, Int))> {
    if !input.starts_with("mul(") {
        return None;
    }
    let mut input = &input[4..];

    let comma = input.find(",")?;
    let num_a = input[..comma].parse::<Int>().ok()?;
    input = &input[comma + 1..];

    let closed_bracket = input.find(")")?;
    let num_b = input[..closed_bracket].parse::<Int>().ok()?;
    input = &input[closed_bracket + 1..];

    Some((input, (num_a, num_b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let sum = parse(test_input).iter().sum::<Int>();
        assert_eq!(sum, 161);
    }

    #[test]
    fn task2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let sum = parse_(test_input).iter().sum::<Int>();
        assert_eq!(sum, 48);
    }
}
