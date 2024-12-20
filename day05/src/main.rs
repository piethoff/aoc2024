use std::fs;

type Int = u64;

fn main() {
    let input = read_input();
    let (orderings, mut updates) = parse_input(&input);
    let sum = get_ordered_middle_sum(&orderings, &updates);

    println!("Task 1: {sum}");

    let unordered_sum = get_unordered_middle_sum(&orderings, &mut updates);

    println!("Task 2: {unordered_sum}");
}

fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("No input.txt found")
}
fn parse_input(input: &str) -> (Vec<(Int, Int)>, Vec<Vec<Int>>) {
    let orderings = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut pair = line.trim().split("|");
            Some((
                pair.next()?.parse::<Int>().ok()?,
                pair.next()?.parse::<Int>().ok()?,
            ))
        })
        .collect::<Vec<_>>();

    let updates = input
        .lines()
        .filter(|line| !line.contains("|") && !line.is_empty())
        .filter_map(|line| {
            line.split(",")
                .map(|num| num.parse::<Int>().ok())
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Vec<_>>();

    (orderings, updates)
}

fn get_first_unordered(orderings: &Vec<(Int, Int)>, update: &Vec<Int>) -> Option<(usize, usize)> {
    // TODO: better search
    orderings
        .iter()
        .filter_map(|ordering| {
            let pos1 = update.iter().position(|&r| r == ordering.0)?;
            let pos2 = update.iter().position(|&r| r == ordering.1)?;
            if pos1 > pos2 {
                Some((pos1, pos2))
            } else {
                None
            }
        })
        .next()
}

fn get_ordered_middle_sum(orderings: &Vec<(Int, Int)>, updates: &Vec<Vec<Int>>) -> Int {
    updates
        .iter()
        .filter_map(|update| {
            if get_first_unordered(orderings, update).is_none() {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}
fn get_unordered_middle_sum(orderings: &Vec<(Int, Int)>, updates: &mut Vec<Vec<Int>>) -> Int {
    updates
        .iter_mut()
        .filter(|update| get_first_unordered(orderings, update).is_some())
        .map(|update| {
            while let Some((pos1, pos2)) = get_first_unordered(orderings, update) {
                update.swap(pos1, pos2);
            }
            update[update.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        let test_input = "\
            47|53\n\
            97|13\n\
            97|61\n\
            97|47\n\
            75|29\n\
            61|13\n\
            75|53\n\
            29|13\n\
            97|29\n\
            53|29\n\
            61|53\n\
            97|53\n\
            61|29\n\
            47|13\n\
            75|47\n\
            97|75\n\
            47|61\n\
            75|61\n\
            47|29\n\
            75|13\n\
            53|13\n\
            \n\
            75,47,61,53,29\n\
            97,61,53,29,13\n\
            75,29,13\n\
            75,97,47,61,53\n\
            61,13,29\n\
            97,13,75,29,47\n\
            ";

        let (orderings, updates) = parse_input(test_input);
        eprintln!("{orderings:?}");
        eprintln!("{updates:?}");

        let sum = get_ordered_middle_sum(&orderings, &updates);

        assert_eq!(sum, 143);
    }

    #[test]
    fn task2() {
        let test_input = "\
            47|53\n\
            97|13\n\
            97|61\n\
            97|47\n\
            75|29\n\
            61|13\n\
            75|53\n\
            29|13\n\
            97|29\n\
            53|29\n\
            61|53\n\
            97|53\n\
            61|29\n\
            47|13\n\
            75|47\n\
            97|75\n\
            47|61\n\
            75|61\n\
            47|29\n\
            75|13\n\
            53|13\n\
            \n\
            75,47,61,53,29\n\
            97,61,53,29,13\n\
            75,29,13\n\
            75,97,47,61,53\n\
            61,13,29\n\
            97,13,75,29,47\n\
            ";

        let (orderings, mut updates) = parse_input(test_input);
        eprintln!("{orderings:?}");
        eprintln!("{updates:?}");

        let sum = get_unordered_middle_sum(&orderings, &mut updates);

        assert_eq!(sum, 123);
    }
}
