use super::day::Day;
pub(crate) struct Day2;
impl Day2 {
    fn parse_input(input: &str) -> Vec<Vec<i32>> {
        input
            .split('\n')
            .filter(|line| line.len() > 1)
            .map(|line| {
                line.split(' ')
                    .filter(|num| num.len() > 0)
                    .map(|c| c.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect()
    }
}
fn is_line_correct(line: &Vec<i32>) -> bool {
    let mut iter = line.iter();
    let first_num = iter.next().unwrap();
    let mut last_num = iter.next().unwrap();
    let direction = if first_num < last_num { -1 } else { 1 };

    if !(1..4).contains(&((first_num - last_num) * direction)) {
        return false;
    }
    for num in iter {
        let diff = (last_num - num) * direction;
        println!("{} {} {}", diff, last_num, num);
        if diff < 4 && diff > 0 {
            last_num = num;
        } else {
            return false;
        }
    }
    true
}
fn is_mutated_line_correct(line: &Vec<i32>) -> bool {
    for i in 0..line.len() {
        let mut mutated_line = line.clone();
        mutated_line.remove(i);
        if is_line_correct(&mutated_line) {
            return true;
        }
    }
    false
}
impl Day for Day2 {
    fn part1(&self, input: &str) -> String {
        let lines = Day2::parse_input(input);
        lines
            .into_iter()
            .filter(|line| is_line_correct(line))
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let lines = Day2::parse_input(input);
        lines
            .into_iter()
            .filter(|line| is_mutated_line_correct(line))
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // blablabla
    fn get_input() -> String {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
        .to_string()
    }
    #[test]
    fn test_1() {
        let day = Day2;

        let result = day.part1(&get_input());
        assert_eq!(result, "2")
    }

    #[test]
    fn test_2() {
        let day = Day2;

        let result = day.part2(&get_input());
        assert_eq!(result, "4")
    }
}
