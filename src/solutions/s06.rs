use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Position,
    dir: Direction,
}

struct Map {
    grid: Vec<Vec<char>>,
    height: i32,
    width: i32,
}

impl Map {
    fn new(input: &str) -> (Self, Position, Direction) {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len() as i32;
        let width = if height > 0 { grid[0].len() as i32 } else { 0 };

        println!("Map dimensions: {}x{}", width, height);

        let mut start_pos = Position { row: 0, col: 0 };
        let mut found = false;

        for (row, line) in grid.iter().enumerate() {
            for (col, &ch) in line.iter().enumerate() {
                if ch == '^' {
                    start_pos = Position { row: row as i32, col: col as i32 };
                    found = true;
                    println!("Found start position at: ({}, {})", row, col);
                    break;
                }
            }
            if found {
                break;
            }
        }

        (Map { grid, height, width }, start_pos, Direction::Up)
    }

    fn is_within_bounds(&self, pos: &Position) -> bool {
        pos.row >= 0 && pos.row < self.height && pos.col >= 0 && pos.col < self.width
    }

    fn is_obstacle(&self, pos: &Position, extra_obstacle: Option<Position>) -> bool {
        if !self.is_within_bounds(pos) {
            return true;
        }
        if let Some(obstacle) = extra_obstacle {
            if *pos == obstacle {
                return true;
            }
        }
        self.grid[pos.row as usize][pos.col as usize] == '#'
    }

    fn simulate_path(&self, start_pos: Position, start_dir: Direction, extra_obstacle: Option<Position>) -> Option<HashSet<Position>> {
        let mut visited = HashSet::new();
        let mut state_history = HashMap::new();
        let mut current_state = State { pos: start_pos, dir: start_dir };
        let max_steps = 10000; // Limit steps to prevent infinite loops

        visited.insert(current_state.pos);

        for step in 0..max_steps {
            state_history.insert(current_state, step);

            let (delta_row, delta_col) = current_state.dir.get_delta();
            let next_pos = Position {
                row: current_state.pos.row + delta_row,
                col: current_state.pos.col + delta_col,
            };

            if !self.is_within_bounds(&next_pos) {
                return None; // Path leads outside
            }

            if self.is_obstacle(&next_pos, extra_obstacle) {
                current_state.dir = current_state.dir.turn_right();
            } else {
                current_state.pos = next_pos;
                visited.insert(current_state.pos);
            }

            if let Some(previous_step) = state_history.get(&current_state) {
                if step - previous_step > 1 {
                    return Some(visited);
                }
            }
        }

        None
    }
}

pub fn solve_both(input: &str) -> (String, String) {
    println!("Starting solution...");
    let (map, start_pos, start_dir) = Map::new(input);

    println!("Solving part 1...");
    let part1 = solve_part1(&map, start_pos, start_dir);
    println!("Part 1 result: {}", part1);

    println!("Solving part 2...");
    let part2 = solve_part2(&map, start_pos, start_dir);
    println!("Part 2 result: {}", part2);

    (part1.to_string(), part2.to_string())
}

fn solve_part1(map: &Map, start_pos: Position, start_dir: Direction) -> usize {
    let mut visited = HashSet::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;
    let mut steps = 0;
    let max_steps = 10000; // Reasonable limit for grid size

    visited.insert(current_pos);

    while steps < max_steps {
        let (delta_row, delta_col) = current_dir.get_delta();
        let next_pos = Position {
            row: current_pos.row + delta_row,
            col: current_pos.col + delta_col,
        };

        if !map.is_within_bounds(&next_pos) {
            break;
        }

        if map.is_obstacle(&next_pos, None) {
            current_dir = current_dir.turn_right();
        } else {
            current_pos = next_pos;
            visited.insert(current_pos);
        }

        steps += 1;
        if steps % 1000 == 0 {
            println!("Step {}, visited {} positions", steps, visited.len());
        }
    }

    println!("Part 1 finished after {} steps", steps);
    visited.len()
}

fn solve_part2(map: &Map, start_pos: Position, start_dir: Direction) -> usize {
    let mut loop_creating_positions = HashSet::new();
    let total_positions = (map.height * map.width) as usize;
    let mut checked = 0;

    for row in 0..map.height {
        for col in 0..map.width {
            let pos = Position { row, col };
            checked += 1;

            if checked % 100 == 0 {
                println!("Checked {}/{} positions ({:.1}%)",
                         checked, total_positions,
                         (checked as f64 / total_positions as f64) * 100.0);
            }

            if pos == start_pos || map.is_obstacle(&pos, None) {
                continue;
            }

            if map.simulate_path(start_pos, start_dir, Some(pos)).is_some() {
                loop_creating_positions.insert(pos);
            }
        }
    }

    println!("Part 2 finished, found {} loop-creating positions", loop_creating_positions.len());
    loop_creating_positions.len()
}