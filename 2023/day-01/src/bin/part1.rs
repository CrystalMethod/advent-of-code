fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let mut result: i32 = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut calibration: Vec<char> = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                calibration.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                calibration.push(c);
                break;
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process(input);
        assert_eq!(result, "142".to_string());
    }
}
