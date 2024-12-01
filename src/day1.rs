use super::day::Day;
use std::iter::zip;
pub(crate) struct Day1;
impl Day1 {
    fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
        input
            .split('\n')
            .filter(|line| line.len() > 1)
            .map(|line| {
                let mut split = line.split("   ");

                let tuple = (split.next().unwrap(), split.next().unwrap());
                tuple
            })
            .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
            .unzip()
    }
}
impl Day for Day1 {
    fn part1(self, input: &str) -> String {
        let (mut l, mut r) = Day1::parse_input(input);

        l.sort();
        r.sort();
        let result: i32 = zip(l, r).map(|(l, r)| (l - r).abs()).sum();
        format!("{}", result)
    }

    fn part2(self, input: &str) -> String {
        let (l, r) = Day1::parse_input(input);
        let result: i32 = l
            .iter()
            .map(|number| {
                let count = r.iter().filter(|other| *other == number).count();
                number * count as i32
            })
            .sum();

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // blablabla
    fn get_input() -> String {
        "3   4
4   3
2   5
1   3
3   9
3   3
"
        .to_string()
    }
    #[test]
    fn test_1() {
        let day = Day1;

        let result = day.part1(&get_input());
        assert!(result == "11")
    }

    #[test]
    fn test_2() {
        let day = Day1;

        let result = day.part2(&get_input());
        assert_eq!(result, "31")
    }
}
