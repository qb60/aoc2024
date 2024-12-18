#![allow(dead_code)]

use std::iter::zip;

pub fn day(input: &str) -> u32 {
    let (mut list1, mut list2) = parse_lists(input);

    list1.sort();
    list2.sort();

    // println!("{:?}", list1);
    // println!("{:?}", list2);

    zip(list1, list2)
        .map(|(num1, num2)| num1.abs_diff(num2))
        .sum()
}

pub fn day_2(input: &str) -> u32 {
    let (list1, list2) = parse_lists(input);
    
    // println!("{:?}", list1);
    // println!("{:?}", list2);

    let sum = list1.iter()
        .map(|num1| num1 * list2.iter().filter(|num2| **num2 == *num1).count() as u32)
        .sum();
    
    sum
}

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let lines = input.lines().filter(|line| !line.is_empty());
    lines.map(|line| line.split_whitespace())
        .map(|split| split.map(|id_str| id_str.parse::<u32>().unwrap()))
        .map(|mut numbers| (numbers.next().unwrap(), numbers.next().unwrap()))
        .unzip()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(11, day(input));
    }

    #[test]
    fn test_day_2() {
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(31, day_2(input));
    }
}
