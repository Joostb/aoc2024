use super::day::Day;
pub(crate) struct Day7;
fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    let input = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|line| line.split_once(": ").unwrap())
        .map(|(l, r)| {
            (
                l.parse().unwrap(),
                r.split(' ').map(|num| num.parse().unwrap()).collect(),
            )
        })
        .collect();
    input
}
impl Day for Day7 {
    fn part1(&self, input: &str) -> String {
        let input = parse_input(&input);

        input
            .iter()
            .filter(|(answer, numbers)| validate_answer(*answer, numbers, false))
            .map(|(anser, _)| anser)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let input = parse_input(&input);

        input
            .iter()
            .filter(|(answer, numbers)| validate_answer(*answer, numbers, true))
            .map(|(anser, _)| anser)
            .sum::<usize>()
            .to_string()
    }
}

fn validate_answer(answer: usize, numbers: &[usize], part_2: bool) -> bool {
    calculation(answer, &numbers[1..], numbers[0], part_2)
}

fn calculation(answer: usize, numbers: &[usize], current_value: usize, part_2: bool) -> bool {
    if numbers.len() == 0 {
        return current_value == answer;
    }
    if part_2 {
        let next_value = numbers[0];
        let concat = format!("{}{}", current_value, next_value);

        if calculation(answer, &numbers[1..], concat.parse().unwrap(), part_2) {
            return true;
        }
    }
    calculation(answer, &numbers[1..], current_value + numbers[0], part_2)
        || calculation(answer, &numbers[1..], current_value * numbers[0], part_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_input() -> String {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
        .to_string()
    }
    #[test]
    fn test_1() {
        let day = Day7;

        let result = day.part1(&get_input());
        assert_eq!(result, "3749")
    }

    #[test]
    fn test_2() {
        let day = Day7;

        let result = day.part2(&get_input());
        assert_eq!(result, "11387")
    }
}
