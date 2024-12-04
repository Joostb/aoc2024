use super::day::Day;
pub(crate) struct Day4;
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .filter(|line| line.len() > 1)
        .map(|line| line.chars().collect())
        .collect()
}
fn count_matches(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
        .sum()
}
fn rows(matrix: &Vec<Vec<char>>) -> Vec<String> {
    matrix.iter().map(|v| v.iter().collect()).collect()
}
fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let length = matrix.len();
    let mut transposed = (0..length).map(|_| vec![]).collect::<Vec<_>>();
    for row in matrix {
        for (num, c) in row.iter().enumerate() {
            transposed[num].push(*c)
        }
    }
    transposed
}
fn rotate(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let length = matrix.len();
    let mut new_rows = Vec::new();
    for col in (0..length).rev() {
        let mut new_row = Vec::new();
        for rw in 0..length {
            new_row.push(matrix[col][rw])
        }
        new_rows.push(new_row)
    }
    new_rows
}
fn columns(matrix: &Vec<Vec<char>>) -> Vec<String> {
    rows(&transpose(matrix))
}
fn diagonals_lr(matrix: &Vec<Vec<char>>) -> Vec<String> {
    let length = matrix.len() * 2 - 1;
    let mut starts = Vec::new();
    for i in (0..length).rev() {
        starts.push((0, i))
    }
    for i in 1..length {
        starts.push((i, 0))
    }
    let mut diagonals = Vec::new();
    for (x, y) in starts {
        let mut x = x;
        let mut y = y;

        let mut diagonal = Vec::new();
        loop {
            if let Some(row) = matrix.get(y) {
                if let Some(value) = row.get(x) {
                    diagonal.push(*value);
                }
            }

            x = x + 1;
            y = y + 1;
            if x > length || y > length {
                break;
            }
        }
        diagonals.push(diagonal);
    }
    diagonals.iter().map(|diag| diag.iter().collect()).collect()
}
fn diagonals_rl(matrix: &Vec<Vec<char>>) -> Vec<String> {
    diagonals_lr(&rotate(matrix))
}

fn match_mini_matrix(matrix: &Vec<Vec<char>>, mini_matrix: &Vec<Vec<char>>) -> usize {
    let bound = matrix.len() - mini_matrix.len();
    let mini_bound = mini_matrix.len();
    let mut matches = 0;
    for x in 0..(bound + 1) {
        for y in 0..(bound + 1) {
            let mut matched = true;
            'mini: for mini_x in 0..mini_bound {
                for mini_y in 0..mini_bound {
                    let value = matrix[x + mini_x][y + mini_y];
                    let mini_value = mini_matrix[mini_x][mini_y];

                    if mini_value == '?' || value == mini_value {
                    } else {
                        matched = false;
                        break 'mini;
                    }
                }
            }
            if matched {
                matches = matches + 1;
            }
        }
    }
    matches
}

impl Day for Day4 {
    fn part1(&self, input: &str) -> String {
        let matrix = parse_input(input);
        let mut count = 0;
        count = count + count_matches(rows(&matrix));
        count = count + count_matches(columns(&matrix));
        count = count + count_matches(diagonals_lr(&matrix));
        count = count + count_matches(diagonals_rl(&matrix));

        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let matrix = parse_input(input);
        let mut count = 0;
        let mini_matrix = vec![
            vec!['M', '?', 'M'],
            vec!['?', 'A', '?'],
            vec!['S', '?', 'S'],
        ];
        // oeps, rotate is eigenlijk flip??
        count = count + match_mini_matrix(&matrix, &mini_matrix);
        let mini_matrix = rotate(&mini_matrix);
        count = count + match_mini_matrix(&matrix, &mini_matrix);
        let mini_matrix = transpose(&mini_matrix);
        count = count + match_mini_matrix(&matrix, &mini_matrix);
        let mini_matrix = transpose(&rotate(&transpose(&mini_matrix)));
        count = count + match_mini_matrix(&matrix, &mini_matrix);

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // blablabla
    fn get_input() -> String {
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"
        .to_string()
    }
    #[test]
    fn test_1() {
        let day = Day4;

        let result = day.part1(&get_input());
        assert_eq!(result, "18")
    }

    #[test]
    fn test_2() {
        let day = Day4;

        let result = day.part2(&get_input());
        assert_eq!(result, "9")
    }
    #[test]
    fn test_input_at_the_edge() {
        let day = Day4;
        let input = "
MMMM
SSSS
AAAA
MMMM
";
        let result = day.part2(&input);
        assert_eq!(result, "2")
    }
}
