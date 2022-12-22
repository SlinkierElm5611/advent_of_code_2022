use std::{fs::File, io::Read, path::Path};

pub fn camp_cleanup_part_one() -> u32 {
    let path = Path::new("day4.txt");
    let mut file: File = match File::open(&path) {
        Err(_) => panic!("could not open file!"),
        Ok(file) => file,
    };
    let mut raw_pairs = String::new();
    match file.read_to_string(&mut raw_pairs) {
        Err(e) => panic!("Error reading from file {}", { e }),
        Ok(lenght) => lenght,
    };
    let pairs: Vec<&str> = raw_pairs.split("\n").collect();
    let mut counter: u32 = 0;
    for pair in pairs {
        let elves: Vec<&str> = pair.split(",").collect();
        let elf_one: Vec<u32> = elves[0]
            .split("-")
            .map(|num| num.parse().unwrap())
            .collect();
        let elf_two: Vec<u32> = elves[1]
            .split("-")
            .map(|num| num.parse().unwrap())
            .collect();
        if elf_one[0] <= elf_two[0] && elf_one[1] >= elf_two[1] {
            counter += 1;
        } else if elf_one[0] >= elf_two[0] && elf_one[1] <= elf_two[1] {
            counter += 1;
        }
    }
    return counter;
}

pub fn camp_cleanup_part_two() -> u32 {
    let path: &Path = Path::new("day4.txt");
    let mut file: File = match File::open(&path) {
        Err(_) => panic!("could not open file!"),
        Ok(file) => file,
    };
    let mut raw_pairs = String::new();
    match file.read_to_string(&mut raw_pairs) {
        Err(e) => panic!("Error reading from file {}", { e }),
        Ok(lenght) => lenght,
    };
    let pairs: Vec<&str> = raw_pairs.split("\n").collect();
    let mut counter: u32 = 0;
    for pair in pairs {
        let elves: Vec<&str> = pair.split(",").collect();
        let elf_one: Vec<u32> = elves[0]
            .split("-")
            .map(|num| num.parse().unwrap())
            .collect();
        let elf_two: Vec<u32> = elves[1]
            .split("-")
            .map(|num| num.parse().unwrap())
            .collect();
        let range_elf_one: Vec<u32> = (elf_one[0]..elf_one[1] + 1).collect();
        let range_elf_two: Vec<u32> = (elf_two[0]..elf_two[1] + 1).collect();
        for plot in range_elf_one {
            if range_elf_two.contains(&plot) {
                counter += 1;
                break;
            }
        }
    }
    return counter;
}
