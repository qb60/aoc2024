#![allow(dead_code)]

use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};

pub fn day(input: &str) -> u32 {
    //012
    //7X3
    //654
    const DIRECTIONS: [Point; 8] = [
        Point { x: -1, y: -1 },
        Point { x: 0, y: -1 },
        Point { x: 1, y: -1 },
        Point { x: 1, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 0, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: -1, y: 0 },
    ];

    const STEPS: [char; 3] = [
        'M', 'A', 'S'
    ];

    let lines: Vec<_> = input.lines().filter(|line| !line.is_empty()).collect();
    let field = Field::try_new(&lines).unwrap();
    let mut sum = 0;

    for y in 0..field.size {
        for x in 0..field.size {
            let current_point = Point::new(x as i32, y as i32);
            if field.get(current_point) != Some('X') {
                continue;
            }

            for direction in DIRECTIONS {
                let mut step = 0;
                while step < 3 {
                    let new_point = current_point + direction * (step + 1) as i32;
                    if field.get(new_point) != Some(STEPS[step]) {
                        break;
                    } else {
                        step += 1;
                    }
                }

                if step == 3 {
                    sum += 1;
                }
            }
        }
    }

    sum
}

pub fn day_2(input: &str) -> u32 {
    //0.1
    //.X.
    //3.2
    const DIRECTIONS: [Point; 4] = [
        Point { x: -1, y: -1 },
        Point { x: 1, y: 1 },
        Point { x: 1, y: -1 },
        Point { x: -1, y: 1 },
    ];

    let lines: Vec<_> = input.lines().filter(|line| !line.is_empty()).collect();
    let field = Field::try_new(&lines).unwrap();
    let mut sum = 0;

    for y in 1..field.size - 1 {
        for x in 1..field.size - 1 {
            let current_point = Point::new(x as i32, y as i32);
            if field.get(current_point) != Some('A') {
                continue;
            }

            let n1 = field.get(current_point + DIRECTIONS[0]);
            let n2 = field.get(current_point + DIRECTIONS[1]);
            let n3 = field.get(current_point + DIRECTIONS[2]);
            let n4 = field.get(current_point + DIRECTIONS[3]);

            if ((n1 == Some('M') && n2 == Some('S')) || (n1 == Some('S') && n2 == Some('M'))) &&
                ((n3 == Some('M') && n4 == Some('S')) || (n3 == Some('S') && n4 == Some('M')))
            {
                sum += 1;
            }
        }
    }

    sum
}

#[derive(Debug, PartialEq, Eq)]
struct Field {
    size: usize,
    data: Vec<Vec<char>>,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self.data.iter().map(|line|
            line.iter().collect()
        ).collect::<Vec<String>>().join("\n");
        write!(f, "{result}")?;

        Ok(())
    }
}

impl Field {
    pub fn try_new(lines: &[&str]) -> Option<Self> {
        let size = lines.len();

        let mut data = Vec::new();

        for line in lines {
            if line.len() != size {
                return None;
            }

            data.push(line.chars().collect());
        }

        Some(Field { size, data })
    }

    pub fn get(&self, point: impl Borrow<Point>) -> Option<char> {
        let point = point.borrow();
        if !self.is_fit(point) {
            return None;
        }

        Some(self.data[point.y as usize][point.x as usize])
    }

    fn is_fit(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.size as i32 && point.y < self.size as i32
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;

        Ok(())
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        assert_eq!(18, day(input));
    }

    #[test]
    fn test_parse_into_field() {
        let lines = ["MMMSXXMASM", "MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX", "XMASAMXAMM", "XXAMMXXAMA", "SMSMSASXSS", "SAXAMASAAA", "MAMMMXMMMM", "MXMXAXMASX"];

        let actual_field = Field::try_new(&lines);
        let expected_field = Field {
            size: 10,
            data: vec![
                vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
                vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
                vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
                vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
                vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
                vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
                vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
                vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
                vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
                vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
            ],
        };

        assert_eq!(Some(expected_field), actual_field);
    }

    #[test]
    fn test_get_from_field() {
        let lines = [
            "MSX",
            "ASM",
            "XXM"
        ];
        let field = Field::try_new(&lines).unwrap();

        assert_eq!(Some('S'), field.get(&Point { x: 1, y: 0 }));
        assert_eq!(Some('S'), field.get(&Point { x: 1, y: 1 }));
        assert_eq!(Some('M'), field.get(&Point { x: 2, y: 2 }));
    }

    #[test]
    fn test_day_small() {
        let input = r#"
XMAS
XXXX
XXXX
XXXX"#;

        assert_eq!(1, day(input));
    }

    #[test]
    fn test_day_2() {
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        assert_eq!(9, day_2(input));
    }
}
