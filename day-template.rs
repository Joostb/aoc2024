use super::day::Day;
pub(crate) struct Day6;
fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let input: Vec<(usize, usize)> = input
        .split('\n')
        .map(|line| line.split_once('|').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    input
}

impl Day for Day6 {
    fn part1(&self, input: &str) -> String {
        todo!()
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_input() -> String {
        "".to_string()
    }
    #[test]
    fn test_1() {
        let day = Day6;

        let result = day.part1(&get_input());
        assert_eq!(result, "143")
    }

    #[test]
    fn test_2() {
        let day = Day6;

        let result = day.part2(&get_input());
        assert_eq!(result, "123")
    }
}
