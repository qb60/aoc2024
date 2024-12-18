#![allow(dead_code)]

#[derive(PartialEq)]
enum StateSimple {
    NotMul,
    M,
    U,
    L,
    OpenBr,
    Num1,
    Comma,
    Num2,
    CloseBr,
}

pub fn day(input: &str) -> u32 {
    use StateSimple::*;

    let mut sum = 0;
    let mut num1_str = String::new();
    let mut num2_str = String::new();
    let mut state = NotMul;

    for c in input.chars() {
        state = match state {
            NotMul if c == 'm' => M,
            M if c == 'u' => U,
            U if c == 'l' => L,
            L if c == '(' => OpenBr,
            OpenBr if c.is_ascii_digit() => Num1,
            Num1 if c.is_ascii_digit() => Num1,
            Num1 if c == ',' => Comma,
            Comma if c.is_ascii_digit() => Num2,
            Num2 if c.is_ascii_digit() => Num2,
            Num2 if c == ')' => CloseBr,
            _ => NotMul,
        };

        match state {
            Num1 => {
                num1_str.push(c);
            }
            Num2 => {
                num2_str.push(c);
            }
            CloseBr => {
                let num1: u16 = num1_str.parse().unwrap();
                let num2: u16 = num2_str.parse().unwrap();
                sum += num1 as u32 * num2 as u32;
                state = NotMul;
            }
            _ => {}
        }

        if state == NotMul {
            num1_str.clear();
            num2_str.clear();
        }
    }

    sum
}

#[derive(PartialEq)]
enum StateComplex {
    Initial,
    M,
    U,
    L,
    OpenBr,
    Num1,
    Comma,
    Num2,
    NumCloseBr,
    D,
    O,
    DoOpenBr,
    DoCloseBr,
    N,
    Apo,
    T,
    DontOpenBr,
    DontCloseBr,
}

#[derive(PartialEq)]
enum MulMode {
    Enabled,
    Disabled,
}

pub fn day_2(input: &str) -> u32 {
    use StateComplex::*;

    let mut sum = 0;
    let mut num1_str = String::new();
    let mut num2_str = String::new();
    let mut state = Initial;
    let mut mode = MulMode::Enabled;

    for c in input.chars() {
        state = match state {
            Initial if c == 'm' && mode == MulMode::Enabled => M,
            Initial if c == 'd' => D,
            M if c == 'u' => U,
            U if c == 'l' => L,
            L if c == '(' => OpenBr,
            OpenBr if c.is_ascii_digit() => Num1,
            Num1 if c.is_ascii_digit() => Num1,
            Num1 if c == ',' => Comma,
            Comma if c.is_ascii_digit() => Num2,
            Num2 if c.is_ascii_digit() => Num2,
            Num2 if c == ')' => NumCloseBr,

            D if c == 'o' => O,
            O if c == '(' => DoOpenBr,
            DoOpenBr if c == ')' => DoCloseBr,

            O if c == 'n' => N,
            N if c == '\'' => Apo,
            Apo if c == 't' => T,
            T if c == '(' => DontOpenBr,
            DontOpenBr if c == ')' => DontCloseBr,

            _ => Initial,
        };

        match state {
            Num1 => {
                num1_str.push(c);
            }
            Num2 => {
                num2_str.push(c);
            }
            NumCloseBr => {
                let num1: u16 = num1_str.parse().unwrap();
                let num2: u16 = num2_str.parse().unwrap();
                sum += num1 as u32 * num2 as u32;
                state = Initial;
            }
            DoCloseBr => {
                mode = MulMode::Enabled;
                state = Initial;
            }
            DontCloseBr => {
                mode = MulMode::Disabled;
                state = Initial;
            }
            _ => {}
        }

        if state == Initial {
            num1_str.clear();
            num2_str.clear();
        }
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(161, day(input));
    }

    #[test]
    fn test_mul_at_end() {
        let input = r#"mul(2,4)mul(6,5)"#;
        assert_eq!(38, day(input));
    }

    #[test]
    fn test_day_2() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(48, day_2(input));
    }
}
