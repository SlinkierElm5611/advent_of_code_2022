use std::{fs::File, io::Read, path::Path};

pub fn rucksack_reorganization_part_one() -> u32 {
    let path = Path::new("day3.txt");
    let mut file: File = match File::open(&path) {
        Err(_) => panic!("could not open file!"),
        Ok(file) => file,
    };
    let mut raw_bags = String::new();
    match file.read_to_string(&mut raw_bags) {
        Err(e) => panic!("Error reading from file {}", { e }),
        Ok(lenght) => lenght,
    };
    let bags: Vec<&str> = raw_bags.split("\n").collect();
    let mut sum_of_priorities: u32 = 0;
    for bag in bags {
        let compartment_one: &[u8] = bag[0..bag.len()/2].as_bytes();
        let compartment_two: &[u8] = bag[bag.len()/2..bag.len()].as_bytes();
        for item in compartment_one{
            if compartment_two.contains(item){
                if *item >= 97{
                    sum_of_priorities += (*item as u32) - 96;
                }else {
                    sum_of_priorities += (*item as u32) - 38;
                }
                break;
            }
        }
    };
    return sum_of_priorities;
}

pub fn rucksack_reorganization_part_two() -> u32 {
    let path = Path::new("day3.txt");
    let mut file: File = match File::open(&path) {
        Err(_) => panic!("could not open file!"),
        Ok(file) => file,
    };
    let mut raw_bags = String::new();
    match file.read_to_string(&mut raw_bags) {
        Err(e) => panic!("Error reading from file {}", { e }),
        Ok(lenght) => lenght,
    };
    let bags: Vec<&str> = raw_bags.split("\n").collect();
    let mut sum_of_priorities: u32 = 0;
    let mut index: u32 = 2;
    loop {
        if (index as usize) >= bags.len(){
            break;
        }
        for item in bags[index as usize].as_bytes(){
            if bags[(index-1) as usize].as_bytes().contains(&item){
                if bags[(index-2) as usize].as_bytes().contains(&item){
                    if *item >= 97{
                        sum_of_priorities += (*item as u32) - 96;
                    }else {
                        sum_of_priorities += (*item as u32) - 38;
                    }
                    break
                }
            }
        }
        index += 3;
    };
    return sum_of_priorities;
}