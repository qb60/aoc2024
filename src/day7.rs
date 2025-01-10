#![allow(dead_code)]

pub fn day(input: &str) -> u64 {
    const OPERATIONS: [Operation; 2] = [Operation::Add, Operation::Mul];
    sum_equations(input, OPERATIONS)
}

pub fn day_2(input: &str) -> u64 {
    const OPERATIONS: [Operation; 3] = [Operation::Add, Operation::Mul, Operation::Concat];
    sum_equations(input, OPERATIONS)
}

fn sum_equations<const N: usize>(input: &str, operations_set: [Operation; N]) -> u64 {
    let input = parse_input(input);
    let sum = input.equations.iter().filter_map(
        |equation: &Equation| if check_equation(equation, operations_set) {
            Some(equation.result)
        } else {
            None
        }
    ).sum();

    for equation in input.equations {
        if check_equation(&equation, operations_set) {}
    }

    sum
}

fn check_equation<const N: usize>(equation: &Equation, operations_set: [Operation; N]) -> bool {
    let operations_iter = OperationsIter::new(equation.operands.len() - 1, operations_set);

    for operations in operations_iter {
        let mut operands_iter = equation.operands.iter();
        let first_operand = operands_iter.next().unwrap();
        let result: u64 = operands_iter.zip(operations.iter()).fold(*first_operand as u64, |acc, (operand, operation)| {
            match operation {
                Operation::Add => { acc + *operand as u64 }
                Operation::Mul => { acc * *operand as u64 }
                Operation::Concat => { format!("{}{}", acc, operand).parse().unwrap() }
            }
        });

        if result == equation.result {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Mul,
    Concat
}

#[derive(Debug)]
struct OperationsIter<const N: usize> {
    operations_set: [Operation; N],
    operations: Vec<usize>,
    is_finished: bool,
}

impl<const N: usize> OperationsIter<N> {
    fn new(size: usize, operations_set: [Operation; N]) -> Self {
        OperationsIter {
            operations_set,
            operations: vec![0; size],
            is_finished: false,
        }
    }
}

impl<const N: usize> Iterator for OperationsIter<N> {
    type Item = Vec<Operation>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        let result = Some(self.operations.iter().map(|digit_idx| self.operations_set[*digit_idx]).collect());

        let mut digit_number = 0;
        let mut carry = 1;
        loop {
            let mut digit_idx = self.operations[digit_number];
            digit_idx += carry;
            if digit_idx == N {
                carry = 1;
                digit_idx = 0;
            } else {
                carry = 0;
            }

            self.operations[digit_number] = digit_idx;
            digit_number += 1;

            if carry == 0 || digit_number == self.operations.len() {
                break;
            }
        }

        if carry == 1 {
            self.is_finished = true;
        }

        result
    }
}

#[derive(Debug, Default, PartialEq)]
struct Equation {
    result: u64,
    operands: Vec<u32>,
}

#[derive(Debug, Default, PartialEq)]
struct Input {
    equations: Vec<Equation>,
}

fn parse_input(input: &str) -> Input {
    let lines: Vec<_> = input.lines().filter(|line| !line.is_empty()).collect();

    let equations = lines.iter().filter_map(|&line|
        if let [result, operands] = line.split(":").collect::<Vec<_>>().as_slice() {
            Some(Equation {
                result: result.parse().unwrap(),
                operands: operands.trim().split(" ").map(
                    |operand| operand.parse().unwrap()
                ).collect(),
            })
        } else {
            None
        }
    ).collect();

    Input {
        equations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(3749, day(input));
    }

    #[test]
    fn test_parse_input() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        let expected_input = Input {
            equations: vec![
                Equation { result: 190, operands: vec![10, 19] },
                Equation { result: 3267, operands: vec![81, 40, 27] },
                Equation { result: 83, operands: vec![17, 5] },
                Equation { result: 156, operands: vec![15, 6] },
                Equation { result: 7290, operands: vec![6, 8, 6, 15] },
                Equation { result: 161011, operands: vec![16, 10, 13] },
                Equation { result: 192, operands: vec![17, 8, 14] },
                Equation { result: 21037, operands: vec![9, 7, 18, 13] },
                Equation { result: 292, operands: vec![11, 6, 16, 20] },
            ],
        };

        let actual_input = parse_input(input);

        assert_eq!(expected_input, actual_input);
    }

    #[test]
    fn test_operations_iter() {
        const OPERATIONS: [Operation; 2] = [Operation::Add, Operation::Mul];

        let mut iter = OperationsIter::new(2, OPERATIONS);

        assert_eq!(iter.next().unwrap(), [Operation::Add, Operation::Add]);
        assert_eq!(iter.next().unwrap(), [Operation::Mul, Operation::Add]);
        assert_eq!(iter.next().unwrap(), [Operation::Add, Operation::Mul]);
        assert_eq!(iter.next().unwrap(), [Operation::Mul, Operation::Mul]);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_check_equation() {
        const OPERATIONS: [Operation; 2] = [Operation::Add, Operation::Mul];

        let good_equation = Equation {
            result: 55,
            operands: vec![11, 4, 11],
        };

        let bad_equation = Equation {
            result: 22,
            operands: vec![5, 3],
        };

        assert!(check_equation(&good_equation, OPERATIONS));
        assert!(!check_equation(&bad_equation, OPERATIONS));
    }

    #[test]
    fn test_check_equation2() {
        const OPERATIONS: [Operation; 3] = [Operation::Add, Operation::Mul, Operation::Concat];

        let good_equation = Equation {
            result: 55,
            operands: vec![5, 5],
        };

        let bad_equation = Equation {
            result: 22,
            operands: vec![5, 3],
        };

        assert!(check_equation(&good_equation, OPERATIONS));
        assert!(!check_equation(&bad_equation, OPERATIONS));
    }

    #[test]
    fn test_day_2() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(11387, day_2(input));
    }
}
