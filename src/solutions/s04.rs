// src/solutions/s04.rs

pub fn solve(input: &str) -> i64 {
    solve_both(input).1
}

pub fn solve_both(input: &str) -> (i64, i64) {
    let grid = parse_grid(input);
    let part1 = count_xmas(&grid);
    let part2 = count_x_mas(&grid);
    (part1, part2)
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

// Part 1
fn count_xmas(grid: &[Vec<char>]) -> i64 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    let directions = [
        (0isize, 1isize), (1, 1), (1, 0), (1, -1),
        (0, -1), (-1, -1), (-1, 0), (-1, 1)
    ];

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != 'X' {
                continue;
            }

            for (dx, dy) in directions.iter() {
                let target = ['X', 'M', 'A', 'S'];
                let mut valid = true;

                for i in 0..4 {
                    let new_row = row as isize + dx * i as isize;
                    let new_col = col as isize + dy * i as isize;

                    if new_row < 0 || new_row >= rows as isize ||
                        new_col < 0 || new_col >= cols as isize {
                        valid = false;
                        break;
                    }

                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if grid[new_row][new_col] != target[i as usize] {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    // Check bounds for X pattern
    if row < 1 || row >= rows - 1 || col < 1 || col >= cols - 1 {
        return false;
    }

    // Get characters for both diagonals
    let mas1 = [
        grid[row - 1][col - 1],
        grid[row][col],
        grid[row + 1][col + 1]
    ];
    let mas2 = [
        grid[row - 1][col + 1],
        grid[row][col],
        grid[row + 1][col - 1]
    ];

    // Check if either diagonal can form MAS (forward or backward)
    let valid1 = mas1 == ['M', 'A', 'S'] || mas1 == ['S', 'A', 'M'];
    let valid2 = mas2 == ['M', 'A', 'S'] || mas2 == ['S', 'A', 'M'];

    valid1 && valid2
}

fn count_x_mas(grid: &[Vec<char>]) -> i64 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check each position that could be the center of an X
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            if grid[row][col] == 'A' && check_mas(grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmas() {
        let input = "MMMSXXMASM\n\
                     MSAMXMSMSA\n\
                     AMXSXMAAMM\n\
                     MSAMASMSMX\n\
                     XMASAMXAMM\n\
                     XXAMMXXAMA\n\
                     SMSMSASXSS\n\
                     SAXAMASAAA\n\
                     MAMMMXMMMM\n\
                     MXMXAXMASX";
        assert_eq!(count_xmas(&parse_grid(input)), 18);
    }

    #[test]
    fn test_x_mas() {
        let input = ".M.S......\n\
                     ..A..MSMS.\n\
                     .M.S.MAA..\n\
                     ..A.ASMSM.\n\
                     .M.S.M....\n\
                     ..........\n\
                     S.S.S.S.S.\n\
                     .A.A.A.A..\n\
                     M.M.M.M.M.\n\
                     ..........";
        assert_eq!(count_x_mas(&parse_grid(input)), 9);
    }
}