use super::day::Day;
pub(crate) struct Day17;
fn parse_input(input: &str) -> Option<Computer> {
    // let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut lines = input.lines();
    let a: Num = lines.next()?.split(' ').last()?.parse().unwrap();
    let b: Num = lines.next()?.split(' ').last()?.parse().unwrap();
    let c: Num = lines.next()?.split(' ').last()?.parse().unwrap();
    lines.next();
    let program = lines
        .next()?
        .split(' ')
        .last()?
        .split(',')
        .map(|opc| opc.parse().unwrap())
        .collect();

    Some(Computer {
        a,
        b,
        c,
        program,
        pointer: 0,
    })
}
type Num = usize;

#[derive(Debug)]
struct Computer {
    a: Num,
    b: Num,
    c: Num,
    program: Vec<Num>,
    pointer: usize,
}

impl Computer {
    fn run(&mut self) -> Vec<Num> {
        let mut output = Vec::new();
        loop {
            let opcode = match self.program.get(self.pointer) {
                Some(op) => op,
                None => break,
            };
            let literal = match self.program.get(self.pointer + 1) {
                Some(op) => op,
                None => panic!(),
            };
            let combo = match literal {
                0..=3 => *literal,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                7 => 9999999999,
                _ => panic!(),
            };
            match opcode {
                0 => {
                    self.a = self.a / (2_usize.pow(combo as u32));
                    self.pointer += 2;
                }
                1 => {
                    self.b = self.b ^ literal;
                    self.pointer += 2;
                }
                2 => {
                    self.b = combo.rem_euclid(8);
                    self.pointer += 2;
                }
                3 => {
                    if self.a == 0 {
                        self.pointer += 2;
                        continue;
                    }
                    self.pointer = *literal;
                }
                4 => {
                    self.b = self.b ^ self.c;
                    self.pointer += 2;
                }
                5 => {
                    output.push(combo.rem_euclid(8));
                    self.pointer += 2;
                }
                6 => {
                    self.b = self.a / (2_usize.pow(combo as u32));
                    self.pointer += 2;
                }
                7 => {
                    self.c = self.a / (2_usize.pow(combo as u32));
                    self.pointer += 2;
                }
                _ => panic!(),
            }
        }
        output
    }
    fn set_a(&mut self, new_a: Num) {
        self.a = new_a;
        self.pointer = 0;
    }
}
impl Day for Day17 {
    fn part1(&self, input: &str) -> String {
        let mut computer = parse_input(input).unwrap();

        let result = computer.run();
        result
            .into_iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn part2(&self, input: &str) -> String {
        let mut computer = parse_input(input).unwrap();
        let desired_output = computer.program.clone();
        let result = recursive(&mut computer, &desired_output, 0);

        println!("{:?}", result);
        let min_a = result
            .iter()
            .map(|bla| {
                let rev: Vec<Num> = bla.iter().rev().cloned().collect();
                get_a_value(&rev)
            })
            .min()
            .unwrap();
        computer.set_a(min_a);
        let output = computer.run();
        if output != desired_output {
            panic!("no solution found");
        }

        min_a.to_string()
    }
}
fn recursive(
    computer: &mut Computer,
    desired_output: &[Num],
    current_index: usize,
) -> Vec<Vec<Num>> {
    let mut possible_solutions = Vec::new();
    if current_index == desired_output.len() - 1 {
        return (0..7).into_iter().map(|num| vec![num]).collect();
    }
    let current_solutions = recursive(computer, desired_output, current_index + 1);

    let mut desired_a_values = Vec::new();
    let desired_len = desired_output.len();
    for _ in 0..desired_len {
        desired_a_values.push(1);
    }

    for current_solution in &current_solutions {
        for (index, num) in current_solution.iter().enumerate() {
            desired_a_values[desired_len - index - 1] = *num;
        }
        for i in 0..=7 {
            desired_a_values[current_index] = i;
            let new_a = get_a_value(&desired_a_values);
            computer.set_a(new_a);
            let output = computer.run();
            if output[current_index..] == desired_output[current_index..] {
                let mut updated_solution = Vec::new();
                for val in current_solution {
                    updated_solution.push(*val);
                }

                updated_solution.push(i);
                possible_solutions.push(updated_solution);
            }
        }
    }
    possible_solutions
}
fn get_a_value(three_bits: &[Num]) -> Num {
    let mut val = 0;
    for (index, three_bit) in three_bits.iter().enumerate() {
        val += three_bit * (8 as Num).pow(index as u32);
    }
    val
}
#[cfg(test)]
mod tests {
    use super::*;
    fn get_input() -> String {
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
        .to_string()
    }
    #[test]
    fn test_1() {
        let day = Day17;

        let result = day.part1(&get_input());
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0")
    }
    #[test]
    fn adv_1() {
        let mut comp = Computer {
            a: 10,
            b: 1,
            c: 0,
            program: vec![0, 5],
            pointer: 0,
        };
        comp.run();
        assert_eq!(5, comp.a);
    }
    #[test]
    fn specific_1() {
        let mut comp = Computer {
            a: 0,
            b: 0,
            c: 9,
            program: vec![2, 6],
            pointer: 0,
        };
        comp.run();
        assert_eq!(1, comp.b);
    }
    #[test]
    fn specific_2() {
        let mut comp = Computer {
            a: 10,
            b: 0,
            c: 9,
            program: vec![5, 0, 5, 1, 5, 4],
            pointer: 0,
        };
        let result = comp.run();
        assert_eq!(vec![0, 1, 2], result);
    }
    #[test]
    fn specific_3() {
        let mut comp = Computer {
            a: 2024,
            b: 0,
            c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            pointer: 0,
        };
        let result = comp.run();
        assert_eq!(0, comp.a);

        assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], result);
    }

    #[test]
    fn specific_4() {
        let mut comp = Computer {
            a: 0,
            b: 29,
            c: 0,
            program: vec![1, 7],
            pointer: 0,
        };
        comp.run();
        assert_eq!(26, comp.b);
    }
    #[test]
    fn specific_5() {
        let mut comp = Computer {
            a: 0,
            b: 2024,
            c: 43690,
            program: vec![4, 0],
            pointer: 0,
        };
        comp.run();
        assert_eq!(44354, comp.b);
    }

    #[test]
    fn test_2() {
        let day = Day17;

        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
        let result = day.part2(&input);
        assert_eq!(result, "117440")
    }
}
