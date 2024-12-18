#![allow(dead_code)]

pub fn day(input: &str) -> u32 {
    let reports = parse_reports(input);
    // println!("{:?}", reports);

    reports.iter().filter(|report| check_report_is_good(report)).count() as u32
}

pub fn day_2(input: &str) -> u32 {
    let reports = parse_reports(input);
    // println!("{:?}", reports);

    reports.iter().filter(|report| {
        if check_report_is_good(report) {
            return true;
        }

        for i in 0..report.len() {
            let mut sub_report = report.to_vec();
            sub_report.remove(i);
            if check_report_is_good(&sub_report) {
                return true;
            }
        }
        
        false
    }).count() as u32
}

fn parse_reports(input: &str) -> Vec<Vec<u32>> {
    let lines = input.lines().filter(|line| !line.is_empty());
    lines.map(
        |line| line.split_whitespace()
            .map(|num_str| num_str.parse().unwrap())
            .collect()
    ).collect()
}

fn check_report_is_good(report: &[u32]) -> bool {
    if report.len() == 1 {
        return true;
    }

    // println!("{:?}", report);

    let ordering = report[1].cmp(&report[0]);
    // println!("{:?}", ordering);

    let windows = report.windows(2);

    for numbers in windows {
        let diff = (numbers[1] as i32 - numbers[0] as i32) * ordering as i32;
        if !(1..=3).contains(&diff) {
            return false;
        }
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reports() {
        let input = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let actual = parse_reports(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_day() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(2, day(input));
    }

    #[test]
    fn test_check_report() {
        let expected = [
            [7, 6, 4, 2, 1],
            [1, 2, 7, 8, 9],
            [9, 7, 6, 2, 1],
            [1, 3, 2, 4, 5],
            [8, 6, 4, 4, 1],
            [1, 3, 6, 7, 9]
        ];

        assert!(check_report_is_good(&expected[0]));
        assert!(!check_report_is_good(&expected[1]));
        assert!(!check_report_is_good(&expected[2]));
        assert!(!check_report_is_good(&expected[3]));
        assert!(!check_report_is_good(&expected[4]));
        assert!(check_report_is_good(&expected[5]));
    }

    #[test]
    fn test_day_1() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(4, day_2(input));
    }
}
