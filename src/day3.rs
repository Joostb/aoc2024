use super::day::Day;
use regex::Regex;
pub(crate) struct Day3;

impl Day for Day3 {
    fn part1(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut muls: Vec<(i32, i32)> = vec![];
        for (_, [l, r]) in re
            .captures_iter(input)
            .map(|m| m.extract())
            .map(|extracted| extracted)
        {
            muls.push((l.parse().unwrap(), r.parse().unwrap()))
        }
        muls.iter().map(|(l, r)| l * r).sum::<i32>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let re = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut muls: Vec<(i32, i32)> = vec![];
        let mut doing = true;
        for caps in re.captures_iter(input) {
            if let Some(do_match) = caps.get(1) {
                println!("Matched do(): {}", do_match.as_str());
                doing = true;
            } else if let Some(dont_match) = caps.get(2) {
                println!("Matched don't(): {}", dont_match.as_str());
                doing = false;
            } else if let (Some(l), Some(r)) = (caps.get(3), caps.get(4)) {
                if doing {
                    muls.push((l.as_str().parse().unwrap(), r.as_str().parse().unwrap()))
                }
            } else {
                println!("oeps");
            }
        }
        muls.iter().map(|(l, r)| l * r).sum::<i32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // blablabla
    fn get_input() -> String {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()
    }
    #[test]
    fn test_1() {
        let day = Day3;

        let result = day.part1(&get_input());
        assert_eq!(result, "161")
    }

    #[test]
    fn test_2() {
        let day = Day3;

        let result = day.part2(&get_input());
        assert_eq!(result, "48")
    }
}
