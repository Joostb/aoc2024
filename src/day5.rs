use std::{cmp::Ordering, collections::HashSet};

use super::day::Day;
pub(crate) struct Day5;
#[derive(Debug)]
struct Input {
    rules: Vec<(usize, usize)>,
    lines: Vec<Vec<usize>>,
}
fn parse_input(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap();
    let lines = split.next().unwrap();

    let rules: Vec<(usize, usize)> = rules
        .split('\n')
        .map(|line| line.split_once('|').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    let lines = lines
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();
    Input { rules, lines }
}
fn expand_rules(rules: Vec<(usize, usize)>, count: usize) -> HashSet<(usize, usize)> {
    let mut current_rules: HashSet<(usize, usize)> = rules.clone().into_iter().collect();

    loop {
        let mut new_rules: HashSet<(usize, usize)> = HashSet::new();
        for (l_1, r_1) in &current_rules {
            new_rules.insert((*l_1, *r_1));
            for (l_2, r_2) in &current_rules {
                if r_1 == l_2 {
                    new_rules.insert((*l_1, *r_2));
                }
                if r_2 == l_1 {
                    new_rules.insert((*l_2, *r_1));
                }
            }
        }

        if new_rules.len() == count * (count - 1) / 2 {
            return new_rules.into_iter().collect();
        }
        current_rules = new_rules;
    }
}

impl Day for Day5 {
    fn part1(&self, input: &str) -> String {
        let input = parse_input(&input);
        input
            .lines
            .iter()
            .filter(|line| validate_line(line, &input))
            .map(|line| line.get(line.len() / 2).unwrap())
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let input = parse_input(&input);
        input
            .lines
            .iter()
            .filter(|line| !validate_line(line, &input))
            .map(|line| fix_line(line, &input))
            .map(|line| *line.get(line.len() / 2).unwrap())
            .sum::<usize>()
            .to_string()
    }
}

fn fix_line(line: &[usize], input: &Input) -> Vec<usize> {
    let rules = input
        .rules
        .iter()
        .filter(|(l, r)| line.contains(l) && line.contains(r))
        .cloned()
        .collect();

    let expanded_rules = expand_rules(rules, line.len());
    let mut line: Vec<usize> = line.iter().cloned().collect();

    line.sort_by(|l, r| -> Ordering {
        if expanded_rules.contains(&(*l, *r)) {
            Ordering::Less
        } else if expanded_rules.contains(&(*r, *l)) {
            Ordering::Greater
        } else {
            panic!("neither greater nor smaller {} {}", l, r)
        }
    });
    line.iter().cloned().collect()
}

fn validate_line(line: &[usize], input: &Input) -> bool {
    for (index, num) in line.iter().enumerate() {
        for other_num in line.iter().skip(index + 1) {
            for (l, r) in input.rules.iter() {
                if r == num && l == other_num {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    // blablabla
    fn get_input() -> String {
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"
        .to_string()
    }
    #[test]
    fn test_1() {
        let day = Day5;

        let result = day.part1(&get_input());
        assert_eq!(result, "143")
    }

    #[test]
    fn test_2() {
        let day = Day5;

        let result = day.part2(&get_input());
        assert_eq!(result, "123")
    }

    #[test]
    fn test_3() {
        let day = Day5;
        let new_input = "1|3
3|2

1,2,3";

        let result = day.part2(&new_input);
        assert_eq!(result, "3")
    }
}
