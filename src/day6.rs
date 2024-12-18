#![allow(dead_code)]

pub fn day(input: &str) -> u32 {
    677
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
}
