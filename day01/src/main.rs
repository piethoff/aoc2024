use std::{fs, iter::zip};

type Int = u64;

fn main() {
    let input = read_input();
    let (vec_a, vec_b) = parse_input(&input);
    let dist_sum = calculate_distances(vec_a.clone(), vec_b.clone())
        .iter()
        .sum::<Int>();

    println!("Task 1: {dist_sum}");

    let sim_sum = calculate_similarity(vec_a, vec_b).iter().sum::<Int>();

    println!("Task 2: {sim_sum}");
}

fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("No input.txt found")
}
fn parse_input(input: &str) -> (Vec<Int>, Vec<Int>) {
    let mut iter = input
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<_>>())
        .filter_map(|words| match words.len() {
            2 => Some((words[0].parse::<Int>().ok()?, words[1].parse::<Int>().ok()?)),
            _ => None,
        });

    let mut vec_a = vec![];
    let mut vec_b = vec![];

    while let Some(value) = iter.next() {
        vec_a.push(value.0);
        vec_b.push(value.1);
    }

    (vec_a, vec_b)
}

fn calculate_distances(mut vec_a: Vec<Int>, mut vec_b: Vec<Int>) -> Vec<Int> {
    vec_a.sort_unstable();
    vec_b.sort_unstable();

    zip(vec_a, vec_b)
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .collect()
}

fn calculate_similarity(vec_a: Vec<Int>, vec_b: Vec<Int>) -> Vec<Int> {
    let summary_a = Summary::from_ids(vec_a);
    let summary_b = Summary::from_ids(vec_b);

    let mut result = vec![0; summary_a.len()];

    let (mut i, mut j) = (0, 0);

    while i < summary_a.len() {
        match summary_b.get(j) {
            Some(Summary { id, mul }) if *id == summary_a[i].id => {
                result[i] = summary_a[i].mul * mul * id;
                i += 1
            }
            Some(Summary { id, .. }) if *id > summary_a[i].id => {
                result[i] = 0;
                i += 1
            }
            Some(Summary { .. }) => j += 1,
            None => break,
        }
    }

    result
}

#[derive(Debug)]
struct Summary {
    id: Int,
    mul: Int,
}
impl Summary {
    fn from_ids(mut ids: Vec<Int>) -> Vec<Self> {
        ids.sort_unstable();

        let mut result = vec![];
        while let Some(last) = ids.pop() {
            match result.last_mut() {
                Some(Summary { id, mul }) if *id == last => *mul += 1,
                _ => result.push(Summary { id: last, mul: 1 }),
            }
        }

        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        let test_input = "3   4\n\
                          4   3\n\
                          2   5\n\
                          1   3\n\
                          3   9\n\
                          3   3\n";

        let (vec_a, vec_b) = parse_input(test_input);
        let sum = calculate_distances(vec_a, vec_b).iter().sum::<Int>();

        assert_eq!(sum, 11);
    }

    #[test]
    fn task2() {
        let test_input = "3   4\n\
                          4   3\n\
                          2   5\n\
                          1   3\n\
                          3   9\n\
                          3   3\n";

        let (vec_a, vec_b) = parse_input(test_input);
        let sum = calculate_similarity(vec_a, vec_b).iter().sum::<Int>();

        assert_eq!(sum, 31);
    }
}
