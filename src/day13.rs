// use regex::Regex;

use super::day::Day;
pub(crate) struct Day13;
// fn parse_input(input: &str) -> Vec<Bla> {
//     let re = Regex::new(r"\d+").unwrap();
//     // let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
//     let input: Vec<Bla> = input
//         .split("\n\n")
//         .filter(|a| a.len() > 3)
//         .map(|block| {
//             let mut iter = re.find_iter(block).map(|m| m.as_str().parse().unwrap());
//             Bla {
//                 a_x: iter.next().unwrap(),

//                 a_y: iter.next().unwrap(),
//                 b_x: iter.next().unwrap(),
//                 b_y: iter.next().unwrap(),
//                 price_x: iter.next().unwrap(),
//                 price_y: iter.next().unwrap(),
//             }
//         })
//         .collect();
//     input
// }
// type Num = usize;
// #[derive(Debug)]
// struct Bla {
//     a_x: Num,
//     a_y: Num,
//     b_x: Num,
//     b_y: Num,
//     price_x: Num,
//     price_y: Num,
// }
impl Day for Day13 {
    fn part1(&self, _input: &str) -> String {
        todo!()
    }

    fn part2(&self, _input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_input() -> String {
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
        .to_string()
    }
    #[test]
    fn test_1() {
        let day = Day13;

        let result = day.part1(&get_input());
        assert_eq!(result, "480")
    }

    #[test]
    fn test_2() {
        let day = Day13;

        let result = day.part2(&get_input());
        assert_eq!(result, "123")
    }
}
