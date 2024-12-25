#![allow(dead_code)]

use std::collections::HashSet;
use std::ops::Add;

const DIRECTIONS_COUNT: usize = 4;
const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash, Eq)]
struct GuardState {
    position: Point,
    direction_index: usize,
}

pub fn day(input: &str) -> u32 {
    let input = parse_input(input);

    let mut visited_cells = HashSet::from([input.start]);
    let mut current_position = GuardState {
        position: input.start,
        direction_index: 0,
    };

    while let Some(new_position) = get_next_guard_state(&current_position, &input) {
        visited_cells.insert(new_position.position);
        current_position = new_position;
    }

    u32::try_from(visited_cells.len()).unwrap_or(0)
}

pub fn day_2(input: &str) -> u32 {
    let input = parse_input(input);

    let mut visited_cells = HashSet::new();
    let mut current_position = GuardState {
        position: input.start,
        direction_index: 0,
    };

    while let Some(new_position) = get_next_guard_state(&current_position, &input) {
        visited_cells.insert(new_position.position);
        current_position = new_position;
    }
    
    let mut loop_count = 0;
    for &cell_to_try in visited_cells.iter() {
        let mut new_input = input.clone();
        new_input.obstacles.insert(cell_to_try);
        
        if detect_cycle(&new_input).is_some() {
            loop_count += 1;
        }
    }

    loop_count
}

fn detect_cycle(input: &Input) -> Option<()> {
    let initial_position = GuardState {
        position: input.start,
        direction_index: 0,
    };
    
    let mut turtle = get_next_guard_state(&initial_position, input)?;
    let mut hare = get_next_guard_state(&turtle, input)?;

    while turtle != hare {
        turtle = get_next_guard_state(&turtle, input)?;
        hare = get_next_guard_state(&get_next_guard_state(&hare, input)?, input)?;
    }
    
    Some(())
}

fn get_next_guard_state(prev_state: &GuardState, input: &Input) -> Option<GuardState> {
    let current_direction = DIRECTIONS[prev_state.direction_index];
    let new_position = prev_state.position + current_direction;
    if new_position.x < 0 || new_position.y < 0
        || new_position.x >= input.size.x
        || new_position.y >= input.size.y {
        return None;
    }

    if input.obstacles.contains(&new_position) {
        let new_direction_idx = (prev_state.direction_index + 1) % DIRECTIONS_COUNT;
        let new_state = GuardState {
            position: prev_state.position,
            direction_index: new_direction_idx,
        };

        get_next_guard_state(&new_state, input)
    } else {
        Some(GuardState {
            position: new_position,
            direction_index: prev_state.direction_index,
        })
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Input {
    size: Point,
    start: Point,
    obstacles: HashSet<Point>,
}

impl Add<i32> for Point {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

fn parse_input(input: &str) -> Input {
    const OBSTACLE: char = '#';
    const START: char = '^';

    let mut size = Point::default();
    let mut start = Point::default();
    let mut obstacles = HashSet::new();

    let lines = (0..).zip(input.lines());
    for (y, line) in lines {
        size.y = y;
        let chars = (0..).zip(line.chars());
        for (x, char) in chars {
            size.x = x;

            match char {
                START => {
                    start = Point { x, y };
                }
                OBSTACLE => {
                    obstacles.insert(Point { x, y });
                }
                _ => {}
            }
        }
    }

    Input {
        size: size + 1,
        start,
        obstacles,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        assert_eq!(41, day(input));
    }

    #[test]
    fn test_parse_input() {
        let input = r#"..#
.^.
#..
.#.
"#;
        let expected_input = Input {
            size: Point { x: 3, y: 4 },
            start: Point { x: 1, y: 1 },
            obstacles: HashSet::from([
                Point { x: 2, y: 0 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 3 },
            ]),
        };

        let actual_input = parse_input(input);

        assert_eq!(expected_input, actual_input);
    }

    #[test]
    fn test_day_2() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        assert_eq!(6, day_2(input));
    }
}
