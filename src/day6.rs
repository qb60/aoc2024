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

pub fn day(input: &str) -> u32 {
    let input = parse_input(input);

    let mut current_direction_idx = 0;
    let mut current_position = input.start;
    let mut visited_cells = HashSet::from([current_position]);
    loop {
        let current_direction = DIRECTIONS[current_direction_idx];
        let new_position = current_position + current_direction;
        if new_position.x < 0 || new_position.y < 0 
            || new_position.x >= input.size.x
            || new_position.y >= input.size.y {
            break;
        }
        
        if input.obstacles.contains(&new_position) {
            current_direction_idx += 1;
            current_direction_idx %= DIRECTIONS_COUNT;
        } else {
            current_position = new_position;
            visited_cells.insert(current_position);
        }
    }

    u32::try_from(visited_cells.len()).unwrap_or(0)
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Default, PartialEq)]
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
}
