/*
    The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

    For example:

    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet

    In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

    Consider your entire calibration document. What is the sum of all of the calibration values?
*/

use crate::inputs::DATA;

fn get_first_digit(s: &str) -> i64 {
    let i = s.find(|c: char| c.is_digit(10)).unwrap();
    let x = s.chars().nth(i).unwrap();
    x.to_digit(10).unwrap() as i64
}

fn get_last_digit(s: &str) -> i64 {
    let i = s.rfind(|c: char| c.is_digit(10)).unwrap();
    let x = s.chars().nth(i).unwrap();
    x.to_digit(10).unwrap() as i64
}

pub fn get_value(s: &str) -> i64 {
    let result = get_first_digit(s) * 10 + get_last_digit(s);
    result
}

pub fn run() {
    let mut sum = 0i64;
    for input in DATA.split("\n") {
        let actual = get_value(input);
        sum = sum + actual;
    }
    println!("Part 1: Sum = {sum}");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_value() {
        let inputs = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let answers = [12, 38, 15, 77];

        for (input, expected) in inputs.iter().zip(answers) {
            let actual = get_value(input);
            assert_eq!(
                expected, actual,
                "Input = '{input}', Expected: {expected}, Actual: {actual}"
            );
        }
    }
}
