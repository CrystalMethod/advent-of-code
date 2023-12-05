fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(_input: &str) -> String {
    "".to_string()
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
        assert_eq!(result, "467835");
    }
}
