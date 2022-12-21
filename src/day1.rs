use std::{fs::File, io::Read, path::Path};

pub fn calorie_counter() -> i64 {
    let path = Path::new("day1.txt");
    let mut file = match File::open(&path) {
        Err(_) => panic!("could not open file!"),
        Ok(file) => file,
    };
    let mut raw_calories = String::new();
    match file.read_to_string(&mut raw_calories) {
        Err(e) => panic!("Error reading from file {}", { e }),
        Ok(lenght) => lenght,
    };
    let calories: Vec<i64> = raw_calories
        .split("\n")
        .map(|x| x.parse::<i64>().unwrap_or_default())
        .collect();
    dbg!(&calories);
    let mut highest_elves: Vec<i64> = vec![0, 0, 0];
    let mut current_elf: i64 = 0;
    for calorie in calories {
        if calorie != 0 {
            current_elf += calorie;
        } else {
            for elf in &highest_elves {
                if &current_elf > elf {
                    highest_elves.remove(0);
                    highest_elves.push(current_elf.clone());
                    highest_elves.sort();
                    break;
                }
            }
            current_elf = 0;
        }
    }
    current_elf = 0;
    let mut current_max = 0;
    for elf in &highest_elves {
        current_elf += elf;
        if elf > &current_max{
            current_max = *elf;
        }
    }
    println!("{}", current_max);
    return current_elf;
}
