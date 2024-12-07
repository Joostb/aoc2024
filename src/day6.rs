use std::collections::HashSet;

use super::day::Day;

pub(crate) struct Day6;
#[derive(Debug, Clone)]
struct Room {
    boxes: HashSet<(i32, i32)>,
    guard_position: (i32, i32),
    boundaries: (i32, i32),
    direction: (i32, i32),
}
fn parse_input(input: &str) -> Room {
    let input: Vec<Vec<char>> = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|line| line.chars().collect())
        .collect();
    let mut room = Room {
        boxes: HashSet::new(),
        guard_position: (0, 0),
        boundaries: (input.len() as i32, input[0].len() as i32),
        direction: (0, -1), //up
    };
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let x = x as i32;
            let y = y as i32;
            match c {
                '^' => {
                    room.guard_position = (x, y);
                }
                '#' => {
                    room.boxes.insert((x, y));
                }
                '.' => (),
                _ => panic!(),
            }
        }
    }
    room
}
fn add(l: (i32, i32), r: (i32, i32)) -> (i32, i32) {
    (l.0 + r.0, l.1 + r.1)
}
impl Day for Day6 {
    fn part1(&self, input: &str) -> String {
        let room: Room = parse_input(&input);
        let (visited, _) = get_path(room);
        visited.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let room: Room = parse_input(&input);
        let mut counter = 0;
        // very dumb, this checks 130 *130 possibilities, instead of 5000 (the box must be on the path) or even less?
        for x in 0..(room.boundaries.0) {
            for y in 0..(room.boundaries.1) {
                if room.boxes.contains(&(x, y)) {
                    continue;
                }
                let mut changed_room = room.clone();
                changed_room.boxes.insert((x, y));
                let (_, has_loop) = get_path(changed_room);
                if has_loop {
                    counter += 1;
                }
            }
        }
        counter.to_string()
    }
}

fn get_path(mut room: Room) -> (HashSet<(i32, i32)>, bool) {
    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut has_loop = false;
    loop {
        let next_position = add(room.guard_position, room.direction);
        if next_position.0 < 0
            || next_position.0 >= room.boundaries.0
            || next_position.1 < 0
            || next_position.1 >= room.boundaries.1
        {
            break;
        }
        if room.boxes.contains(&next_position) {
            room.direction = match room.direction {
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (0, -1) => (1, 0),
                _ => panic!(),
            };
            continue;
        }
        if visited.contains(&(next_position, room.direction)) {
            has_loop = true;
            break;
        }
        visited.insert((next_position, room.direction));
        room.guard_position = next_position;
    }
    (visited.iter().map(|(l, _)| *l).collect(), has_loop)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_input() -> String {
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
        .to_string()
    }
    #[test]
    fn test_1() {
        let day = Day6;

        let result = day.part1(&get_input());
        assert_eq!(result, "41")
    }

    #[test]
    fn test_2() {
        let day = Day6;

        let result = day.part2(&get_input());
        assert_eq!(result, "6")
    }
}
