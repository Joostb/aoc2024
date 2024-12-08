use std::collections::{HashMap, HashSet};

use super::day::Day;
pub(crate) struct Day8;
fn parse_input(input: &str) -> ((i32, i32), HashMap<char, Vec<(i32, i32)>>) {
    let mut map = HashMap::new();
    let splitted: Vec<&str> = input.split('\n').filter(|l| l.len() > 0).collect();

    let (y_max, x_max) = (splitted.len() as i32, splitted[0].len() as i32);
    let input: Vec<(char, (i32, i32))> = input
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (c, (x as i32, y as i32)))
                .collect::<Vec<(char, (i32, i32))>>()
        })
        .flatten()
        .collect();
    for (c, coordinates) in input {
        map.entry(c)
            .and_modify(|v: &mut Vec<(i32, i32)>| {
                v.push(coordinates);
            })
            .or_insert(vec![coordinates]);
    }
    ((x_max, y_max), map)
}

impl Day for Day8 {
    fn part1(&self, input: &str) -> String {
        let ((x_max, y_max), map) = parse_input(&input);
        let mut anti_node = HashSet::new();
        map.iter().for_each(|(_k, antennas)| {
            for (x_1, y_1) in antennas {
                for (x_2, y_2) in antennas {
                    let x_diff = x_1 - x_2;
                    let y_diff = y_1 - y_2;
                    if x_diff == 0 && y_diff == 0 {
                        continue;
                    }

                    anti_node.insert((x_1 + x_diff, y_1 + y_diff));
                    anti_node.insert((x_2 - x_diff, y_2 - y_diff));
                }
            }
        });
        let filtered: Vec<&(i32, i32)> = anti_node
            .iter()
            .filter(|(x, y)| x >= &0 && y >= &0 && x < &x_max && y < &y_max)
            .collect();
        filtered.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ((x_max, y_max), map) = parse_input(&input);
        let mut anti_node: HashSet<(i32, i32)> = HashSet::new();
        map.iter().for_each(|(_k, antennas)| {
            for (x_1, y_1) in antennas {
                anti_node.insert((*x_1, *y_1));
                for (x_2, y_2) in antennas {
                    let x_diff = x_1 - x_2;
                    let y_diff = y_1 - y_2;
                    if x_diff == 0 && y_diff == 0 {
                        continue;
                    }

                    let mut insert_1 = (x_1 + x_diff, y_1 + y_diff);

                    while in_grid(&insert_1.0, &insert_1.1, x_max, y_max) {
                        anti_node.insert(insert_1);
                        insert_1 = (insert_1.0 - x_diff, insert_1.1 - y_diff);
                    }
                    let mut insert_2 = (x_2 - x_diff, y_2 - y_diff);
                    while in_grid(&insert_2.0, &insert_2.1, x_max, y_max) {
                        anti_node.insert(insert_2);
                        insert_2 = (insert_2.0 - x_diff, insert_2.1 - y_diff);
                    }
                }
            }
        });
        let filtered: Vec<&(i32, i32)> = anti_node
            .iter()
            .filter(|(x, y)| in_grid(x, y, x_max, y_max))
            .collect();
        filtered.len().to_string()
    }
}

fn in_grid(x: &i32, y: &i32, x_max: i32, y_max: i32) -> bool {
    x >= &0 && y >= &0 && x < &x_max && y < &y_max
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_input() -> String {
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
        .to_string()
    }
    #[test]
    fn test_1() {
        let day = Day8;

        let result = day.part1(&get_input());
        assert_eq!(result, "14")
    }

    #[test]
    fn test_1a() {
        let day = Day8;

        let result = day.part1(
            &"....
....
.cc.
....",
        );
        assert_eq!(result, "2")
    }

    #[test]
    fn test_2() {
        let day = Day8;

        let result = day.part2(&get_input());
        assert_eq!(result, "34")
    }
    #[test]
    fn test_3() {
        let day = Day8;

        let result = day.part2(&"cc..");
        assert_eq!(result, "4")
    }
}
