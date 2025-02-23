#![allow(dead_code)]

use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul, Sub};
use itertools::Itertools;
use crate::multimap::MultiMap;

pub fn day(input: &str) -> u32 {
    let input = parse_input(input);

    let mut antinodes = HashSet::new();

    for group in input.antennas_groups {
        for (a1, a2) in group.iter().tuple_combinations() {
            let x1 = a1 * 2 - a2;
            let x2 = a2 * 2 - a1;

            if is_point_in(&x1, &input.size) {
                antinodes.insert(x1);
            }

            if is_point_in(&x2, &input.size) {
                antinodes.insert(x2);
            }
        }
    }

    antinodes.len() as u32
}

pub fn day_2(input: &str) -> u32 {
    let input = parse_input(input);

    let mut antinodes = HashSet::new();

    for group in input.antennas_groups {
        for (a1, a2) in group.iter().tuple_combinations() {
            add_points(a1, a1 - a2, &input.size, &mut antinodes);
            add_points(a2, a2 - a1, &input.size, &mut antinodes);
        }
    }

    antinodes.len() as u32
}

fn is_point_in(point: &Point, size: &Point) -> bool {
    point.x >= 0 && point.y >= 0
        && point.x < size.x
        && point.y < size.y
}

fn add_points<T: Borrow<Point>>(initial_point: &Point, step: T, size: &Point, set_to_add: &mut HashSet<Point>) {
    let mut current_point = *initial_point;
    while is_point_in(&current_point, size) {
        set_to_add.insert(current_point);
        current_point += step.borrow();
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<i32> for Point {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}

impl<T: Borrow<Point>> AddAssign<T> for Point {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.borrow().x;
        self.y += rhs.borrow().y;
    }
}

impl<T: Borrow<Point>> Sub<T> for Point {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self { x: self.x - rhs.borrow().x, y: self.y - rhs.borrow().y }
    }
}

impl<T: Borrow<Point>> Sub<T> for &Point {
    type Output = Point;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output { x: self.x - rhs.borrow().x, y: self.y - rhs.borrow().y }
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<i32> for &Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs }
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    size: Point,
    antennas_groups: Vec<Vec<Point>>,
}

fn parse_input(input: &str) -> Input {
    const EMPTY_CELL: char = '.';
    let mut size = Point::default();
    let mut antennas_groups = MultiMap::new();

    let lines = (0..).zip(input.lines());
    for (y, line) in lines {
        size.y = y;
        let chars = (0..).zip(line.chars());
        for (x, char) in chars {
            size.x = x;
            if char != EMPTY_CELL {
                antennas_groups.insert(char, Point { x, y });
            }
        }
    }

    Input {
        size: size + 1,
        antennas_groups: antennas_groups.grouped_values().map(|(_, points)| points.copied().collect()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = r#"............
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
............"#;

        assert_eq!(14, day(input));
    }

    #[test]
    fn test_parse_input() {
        let input = r#"..a..
..a..
.a0..
...x.
.0.x."#;

        let expected_input = Input {
            size: Point { x: 5, y: 5 },
            antennas_groups: vec![
                vec![
                    Point { x: 1, y: 2 },
                    Point { x: 2, y: 0 },
                    Point { x: 2, y: 1 },
                ],
                vec![
                    Point { x: 1, y: 4 },
                    Point { x: 2, y: 2 },
                ],
                vec![
                    Point { x: 3, y: 3 },
                    Point { x: 3, y: 4 },
                ]],
        };

        let mut actual_input = parse_input(input);
        actual_input.antennas_groups.iter_mut().for_each(|v| v.sort());
        actual_input.antennas_groups.sort();

        assert_eq!(actual_input, expected_input);
    }

    #[test]
    fn test_day_2() {
        let input = r#"............
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
............"#;

        assert_eq!(34, day_2(input));
    }
}
