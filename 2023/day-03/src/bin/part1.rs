use std::u32;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let line_width = input.find(char::is_whitespace).unwrap();
    let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let symbols: Vec<usize> = input
        .match_indices(|c: char| !(c.is_ascii_digit() || c == '.'))
        .map(|m| m.0)
        .collect();

    let mut number: String = String::new();
    let mut found: bool = false;
    let mut adjacent_symbol = false;

    let mut parts: Vec<u32> = Vec::new();

    for (i, char) in input.chars().enumerate() {
        let mut process_number = || {
            if found && adjacent_symbol {
                parts.push(number.parse().unwrap());
            }
            number = String::new();
            found = false;
            adjacent_symbol = false;
        };

        if i % line_width == 0 {
            process_number()
        }

        // start or continue a number
        if char.is_ascii_digit() {
            found = true;
            number.push(char);
            if !adjacent_symbol {
                adjacent_symbol = (i >= line_width
                    && i % line_width != 0
                    && symbols.contains(&(i - line_width - 1))) // top left
                || (i>= line_width && symbols.contains(&(i - line_width))) // top
                || (i >= line_width && i % line_width != line_width - 1 && symbols.contains(&(i - line_width + 1))) // top right
                || (i % line_width != 0 && symbols.contains(&(i - 1))) // left
                || (i % line_width != line_width - 1 && symbols.contains(&(i + 1))) // right
                || (i < input.len() - line_width && i % line_width != 0 && symbols.contains(&(i + line_width - 1))) // bottom left
                || (i < input.len() - line_width && symbols.contains(&(i + line_width))) // bottom
                || (i < input.len() - line_width && i % line_width != line_width - 1 && symbols.contains(&(i + line_width + 1)));
                // bottom right
            }
        } else {
            process_number();
        }
    }

    // input ends with a number
    if found && adjacent_symbol {
        parts.push(number.parse().unwrap());
    }

    parts.iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(input);
        assert_eq!(result, "4361");
    }
}
