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
    split.next().unwrap();
    let record: &str = split.next().unwrap();
    check_record(record)
}

fn check_record(record: &str) -> u32 {
    let tries: Vec<_> = record.split(';').map(|t| t.trim()).collect();
    let mut min_cubes: (u32, u32, u32) = (0, 0, 0);
    for tri in tries.iter() {
        let (min_red, min_green, min_blue) = check_try(tri);
        if min_red > min_cubes.0 {
            min_cubes.0 = min_red;
        }
        if min_green > min_cubes.1 {
            min_cubes.1 = min_green;
        }
        if min_blue > min_cubes.2 {
            min_cubes.2 = min_blue;
        }
    }
    min_cubes.0 * min_cubes.1 * min_cubes.2
}

fn check_try(tri: &str) -> (u32, u32, u32) {
    let cubes: Vec<_> = tri.split(',').map(|c| c.trim()).collect();
    let mut min_cubes: (u32, u32, u32) = (0, 0, 0);
    for cube in cubes.iter() {
        if cube.ends_with("red") {
            min_cubes.0 = cube
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
        } else if cube.ends_with("green") {
            min_cubes.1 = cube
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
        } else if cube.ends_with("blue") {
            min_cubes.2 = cube
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }
    }
    min_cubes
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
        assert_eq!(result, "2286".to_string());
    }
}
