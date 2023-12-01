fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let numwords = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let lines: Vec<_> = input.lines().collect();
    let mut result: i32 = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut calibration: Vec<char> = Vec::new();
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                calibration.push(c);
                break;
            } else {
                chars.push(c);
                let mut word: String = chars.iter().collect::<String>();
                for numword in numwords {
                    if word.as_str().ends_with(&numword) {
                        word = numword.to_string();
                        match word.as_str() {
                            "one" => calibration.push('1'),
                            "two" => calibration.push('2'),
                            "three" => calibration.push('3'),
                            "four" => calibration.push('4'),
                            "five" => calibration.push('5'),
                            "six" => calibration.push('6'),
                            "seven" => calibration.push('7'),
                            "eight" => calibration.push('8'),
                            "nine" => calibration.push('9'),
                            _ => (),
                        }
                        break;
                    }
                }
                if calibration.len() == 1 {
                    break;
                }
            }
        }
        chars.clear();
        for c in line.chars().rev() {
            if c.is_numeric() {
                calibration.push(c);
                break;
            } else {
                chars.push(c);
                let mut word: String = chars.iter().rev().collect::<String>();
                for numword in numwords {
                    if word.as_str().starts_with(&numword) {
                        word = numword.to_string();
                        match word.as_str() {
                            "one" => calibration.push('1'),
                            "two" => calibration.push('2'),
                            "three" => calibration.push('3'),
                            "four" => calibration.push('4'),
                            "five" => calibration.push('5'),
                            "six" => calibration.push('6'),
                            "seven" => calibration.push('7'),
                            "eight" => calibration.push('8'),
                            "nine" => calibration.push('9'),
                            _ => (),
                        }
                        break;
                    }
                }
                if calibration.len() == 2 {
                    break;
                }
            }
        }
        result += calibration
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
    }
    result.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process(input);
        assert_eq!(result, "281".to_string());
    }
}
