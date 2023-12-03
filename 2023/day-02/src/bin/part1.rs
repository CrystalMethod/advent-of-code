use std::u32;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let mut result: u32 = 0;
    for line in lines.iter() {
        result += check_line(line);
    }
    result.to_string()
}

fn check_line(line: &str) -> u32 {
    let mut split = line.split(':');
    let game: &str = split.next().unwrap();
    let record: &str = split.next().unwrap();
    if check_record(record) {
        value(game)
    } else {
        0
    }
}

fn check_record(record: &str) -> bool {
    let tries: Vec<_> = record.split(';').map(|t| t.trim()).collect();
    let mut result: bool = true;
    for tri in tries.iter() {
        result = check_try(tri);
        if !result {
            break;
        }
    }
    result
}

fn check_try(tri: &str) -> bool {
    let cubes: Vec<_> = tri.split(',').map(|c| c.trim()).collect();
    for cube in cubes.iter() {
        if cube.ends_with("red") {
            let count: u32 = cube
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            if count > 12 {
                return false;
            }
        } else if cube.ends_with("green") {
            let count: u32 = cube
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            if count > 13 {
                return false;
            }
        } else if cube.ends_with("blue") {
            let count: u32 = cube
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            if count > 14 {
                return false;
            }
        }
    }
    true
}

fn value(s: &str) -> u32 {
    s["Game ".len()..].parse::<u32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(input);
        assert_eq!(result, "8".to_string());
    }
}
