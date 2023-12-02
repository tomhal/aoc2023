/*
    Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

    Equipped with this new information, you now need to find the real first and last digit on each line. For example:

    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen

    In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
*/

use crate::{inputs::INPUT_DAY1_1, part1::get_value};

fn replace_all(s: &str) -> String {
    let replacements = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut input = String::from(s);
    let mut result = String::new();

    while input.len() > 0 {
        let mut replaced = false;

        // println!("input.len() = {}", input.len());
        // println!("input = '{input}', result = '{result}'");
        for (i, replacement) in replacements.iter().enumerate() {
            if input.starts_with(replacement) {
                let c = char::from_u32('0' as u32 + i as u32 + 1u32).unwrap();
                // println!("Replacing '{replacement}' with '{c}'");
                result.push(c);
                let len = replacement.len();
                input = input[1..].to_string();
                // println!("update: input = '{input}', result = '{result}'");

                replaced = true;
                continue;
            }
        }

        if !replaced {
            // If there were no replacements, copy the first char to result
            result.push(input.chars().nth(0).unwrap());
            input.remove(0);
        }
    }

    result
}

pub fn run() {
    let mut sum = 0;
    for input in INPUT_DAY1_1.split("\n") {
        let modified_input = replace_all(input);
        let actual = get_value(modified_input.as_str());
        sum = sum + actual;

        println!("{sum}\n'{input}'\n'{modified_input}'\n{actual}\n");
    }
    println!("Part 2: Sum = {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{inputs, part1::get_value};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_value() {
        let inputs = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let answers = [29, 83, 13, 24, 42, 14, 76];
        let mut sum = 0;
        for (input, expected) in inputs.split("\n").zip(answers) {
            let modified_input = replace_all(input);
            println!("'{input}' -> '{modified_input}'");
            let actual = get_value(modified_input.as_str());
            assert_eq!(
                expected, actual,
                "Input = '{input}', Expected: {expected}, Actual: {actual}"
            );

            sum = sum + actual;
        }

        assert_eq!(281, sum);
    }

    #[test]
    fn test_replace_all() {
        let inputs = "oneight";

        let answers = [18];
        for (input, expected) in inputs.split("\n").zip(answers) {
            let modified_input = replace_all(input);
            println!("'{input}' -> '{modified_input}'");
            let actual = get_value(modified_input.as_str());
            assert_eq!(
                expected, actual,
                "Input = '{input}', Expected: {expected}, Actual: {actual}"
            );
        }
    }
}
