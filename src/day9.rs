use std::{fs::File, io::Read, path::Path};

pub fn rope_bridge_part_one() -> usize {
    let path: &Path = Path::new("day9.txt");
    let mut data_file: File = File::open(path).expect("Error opening file");
    let mut instructions: String = String::new();
    data_file
        .read_to_string(&mut instructions)
        .expect("Error reading from open file!");
    const NUMBER_OF_KNOTS: usize = 2;
    let mut locations_visited: std::collections::HashSet<(i64, i64)> =
        std::collections::HashSet::new();
    let mut rope: [[i64; 2]; NUMBER_OF_KNOTS] = [[0; 2]; NUMBER_OF_KNOTS];
    for instruction in instructions.lines() {
        let processed_instruction: &[&str] = &instruction.split(" ").collect::<Vec<&str>>();
        for _ in 0..processed_instruction[1].parse::<usize>().unwrap_or(0) {
            match processed_instruction[0] {
                "U" => {
                    rope[0][1] += 1;
                }
                "D" => {
                    rope[0][1] -= 1;
                }
                "R" => {
                    rope[0][0] += 1;
                }
                "L" => {
                    rope[0][0] -= 1;
                }
                _ => {
                    panic!("Improper input!");
                }
            };
            for index in 1..NUMBER_OF_KNOTS {
                if (rope[index - 1][0] - rope[index][0]).abs() > 1
                    || (rope[index - 1][1] - rope[index][1]).abs() > 1
                {
                    rope[index][0] += ((rope[index - 1][0] > rope[index][0]) as i64)
                        - ((rope[index][0] > rope[index - 1][0]) as i64);
                    rope[index][1] += ((rope[index - 1][1] > rope[index][1]) as i64)
                        - ((rope[index][1] > rope[index - 1][1]) as i64);
                }
            }
            locations_visited.insert((rope[NUMBER_OF_KNOTS - 1][0], rope[NUMBER_OF_KNOTS - 1][1]));
        }
    }
    return locations_visited.len();
}

pub fn rope_bridge_part_two() -> usize {
    let path: &Path = Path::new("day9.txt");
    let mut data_file: File = File::open(path).expect("Error opening file");
    let mut instructions: String = String::new();
    data_file
        .read_to_string(&mut instructions)
        .expect("Error reading from open file!");
    const NUMBER_OF_KNOTS: usize = 10;
    let mut locations_visited: std::collections::HashSet<(i64, i64)> =
        std::collections::HashSet::new();
    let mut rope: [[i64; 2]; NUMBER_OF_KNOTS] = [[0; 2]; NUMBER_OF_KNOTS];
    for instruction in instructions.lines() {
        let processed_instruction: &[&str] = &instruction.split(" ").collect::<Vec<&str>>();
        for _ in 0..processed_instruction[1].parse::<usize>().unwrap_or(0) {
            match processed_instruction[0] {
                "U" => {
                    rope[0][1] += 1;
                }
                "D" => {
                    rope[0][1] -= 1;
                }
                "R" => {
                    rope[0][0] += 1;
                }
                "L" => {
                    rope[0][0] -= 1;
                }
                _ => {
                    panic!("Improper input!");
                }
            };
            for index in 1..NUMBER_OF_KNOTS {
                if (rope[index - 1][0] - rope[index][0]).abs() > 1
                    || (rope[index - 1][1] - rope[index][1]).abs() > 1
                {
                    rope[index][0] += ((rope[index - 1][0] > rope[index][0]) as i64)
                        - ((rope[index][0] > rope[index - 1][0]) as i64);
                    rope[index][1] += ((rope[index - 1][1] > rope[index][1]) as i64)
                        - ((rope[index][1] > rope[index - 1][1]) as i64);
                }
            }
            locations_visited.insert((rope[NUMBER_OF_KNOTS - 1][0], rope[NUMBER_OF_KNOTS - 1][1]));
        }
    }
    return locations_visited.len();
}