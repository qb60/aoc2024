#![allow(dead_code)]

use crate::multimap::MultiMap;

pub fn day(input: &str) -> u32 {
    let input = parse_input(input);
    let mut sum: u32 = 0;

    for update in input.updates {
        let mut bad = false;
        let pairs = update.windows(2);
        for pair in pairs {
            if let [n1, n2] = pair {
                if input.rules.contains(n2, n1) {
                    bad = true;
                    break;
                }
            }
        }

        if !bad {
            sum += update[update.len() / 2] as u32;
        }
    }

    sum
}

pub fn day_2(input: &str) -> u32 {
    let input = parse_input(input);
    let mut sum: u32 = 0;

    for update in input.updates {
        let rules = filter_relevant_rules(&input.rules, &update);

        let mut bad = false;
        let pairs = update.windows(2);
        for pair in pairs {
            if let [n1, n2] = pair {
                if input.rules.contains(n2, n1) {
                    bad = true;
                    break;
                }
            }
        }

        if bad {
            let fixed_update = sort_update(&update, &rules);
            sum += fixed_update[fixed_update.len() / 2] as u32;
        }
    }

    sum
}

fn sort_update(update: &[Page], rules: &Rules) -> Vec<Page> {
    let mut rules = (*rules).clone();

    let first = update.iter().find(|page|
        rules.values().all(|next| next != *page)
    ).unwrap();

    let mut result = vec![];
    let mut pages_to_check = vec![*first];

    while let Some(current_page) = pages_to_check.pop() {
        result.push(current_page);
        let next_pages = rules.get(&current_page);
        if let Some(next_pages) = next_pages {
            let next_pages = next_pages.cloned().collect::<Vec<_>>();
            for next_page in next_pages {
                rules.remove(&current_page, &next_page);
                if !rules.values().any(|page| *page == next_page) {
                    pages_to_check.push(next_page);
                }
            }
        }
    }

    result
}

fn filter_relevant_rules(rules: &Rules, update: &[u8]) -> Rules {
    rules.iter()
        .filter(|(prev, posts)| update.contains(prev) && update.contains(posts))
        .map(|(prev, posts)| (*prev, *posts))
        .collect()
}

type Page = u8;
type Rules = MultiMap<Page, Page>;

#[derive(Debug, PartialEq, Eq)]
struct Input {
    rules: Rules,
    updates: Vec<Vec<Page>>,
}

fn parse_input(input: &str) -> Input {
    let mut rules = Rules::new();
    let mut updates = Vec::new();

    let mut lines = input.lines();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let mut key_value = line.split('|');
        let key = key_value.next().unwrap().parse::<u8>().unwrap();
        let value = key_value.next().unwrap().parse::<u8>().unwrap();

        rules.insert(key, value);
    }

    for line in lines {
        let update = line.split(',').map(|num_str| num_str.parse::<u8>().unwrap()).collect::<Vec<_>>();
        updates.push(update);
    }

    Input { rules, updates }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

        assert_eq!(143, day(input));
    }

    #[test]
    fn test_parse_input() {
        let input = r#"11|22
22|33
33|55
22|44

11,22,55
61,55,67
81,32,19,34,35
"#;

        let expected_parsed_input = Input {
            rules: Rules::from([
                (11, 22),
                (22, 33),
                (22, 44),
                (33, 55),
            ]),
            updates: vec![
                vec![11, 22, 55],
                vec![61, 55, 67],
                vec![81, 32, 19, 34, 35]
            ],
        };

        let actual_parsed_input = parse_input(input);

        assert_eq!(expected_parsed_input, actual_parsed_input);
    }

    #[test]
    fn test_day_2() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

        assert_eq!(123, day_2(input));
    }

    #[test]
    fn test_sort_update() {
        let update = vec![11, 44, 33, 00, 22];
        let rules = Rules::from([
            (00, 22),
            (00, 11),
            (11, 44),
            (11, 33),
            (11, 22),
            (22, 44),
            (22, 33),
            (33, 44),
        ]);

        assert_eq!(vec![00, 11, 22, 33, 44], sort_update(&update, &rules));
    }
}
