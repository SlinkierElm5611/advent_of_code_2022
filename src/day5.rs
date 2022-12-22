use std::{fs::File, io::Read, path::Path};

pub fn supply_stacks_part_one() -> String {
    let path: &Path = Path::new("day5.txt");
    let mut file: File = match File::open(&path) {
        Err(_) => panic!("could not open file!"),
        Ok(file) => file,
    };
    let mut raw_input = String::new();
    match file.read_to_string(&mut raw_input) {
        Err(e) => panic!("Error reading from file {}", { e }),
        Ok(lenght) => lenght,
    };
    let input: Vec<&str> = raw_input.split("\n").collect();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut index: u32 = 0;
    loop {
        if (index as usize) >= input.len() || *input.get(index as usize).unwrap() == "" {
            break;
        }
        index += 1;
    }
    for i in 0..(index - 1) as usize {
        let mut counter: u32 = 1;
        loop {
            if counter >= input[i].len() as u32 {
                break;
            }
            if i == 0 {
                stacks.push(Vec::new());
            }
            let item: char = input[i].chars().collect::<Vec<char>>()[counter as usize];
            if item != ' ' {
                stacks[(counter / 4) as usize].push(item);
            }
            counter += 4;
        }
    }
    for i in 0..stacks.len(){
        stacks[i].reverse();
    }
    for i in ((index + 1) as usize)..input.len() {
        let instruction: Vec<&str> = input[i].split(" ").collect();
        for _ in 0..instruction[1].parse::<usize>().unwrap(){
            let source: usize = instruction[3].parse::<usize>().unwrap() - 1;
            let destination: usize = instruction[5].parse::<usize>().unwrap() - 1;
            let item: char = stacks[source].pop().unwrap();
            stacks[destination].push(item);
        }
    }
    let mut output_string: String = String::new();
    for stack in stacks{
        output_string.push(stack[stack.len()-1]);
    }
    return output_string;
}

pub fn supply_stacks_part_two() -> String {
    let path: &Path = Path::new("day5.txt");
    let mut file: File = match File::open(&path) {
        Err(_) => panic!("could not open file!"),
        Ok(file) => file,
    };
    let mut raw_input = String::new();
    match file.read_to_string(&mut raw_input) {
        Err(e) => panic!("Error reading from file {}", { e }),
        Ok(lenght) => lenght,
    };
    let input: Vec<&str> = raw_input.split("\n").collect();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut index: u32 = 0;
    loop {
        if (index as usize) >= input.len() || *input.get(index as usize).unwrap() == "" {
            break;
        }
        index += 1;
    }
    for i in 0..(index - 1) as usize {
        let mut counter: u32 = 1;
        loop {
            if counter >= input[i].len() as u32 {
                break;
            }
            if i == 0 {
                stacks.push(Vec::new());
            }
            let item: char = input[i].chars().collect::<Vec<char>>()[counter as usize];
            if item != ' ' {
                stacks[(counter / 4) as usize].push(item);
            }
            counter += 4;
        }
    }
    for i in 0..stacks.len(){
        stacks[i].reverse();
    }
    for i in ((index + 1) as usize)..input.len() {
        let instruction: Vec<&str> = input[i].split(" ").collect();
        let source: usize = instruction[3].parse::<usize>().unwrap() - 1;
        let destination: usize = instruction[5].parse::<usize>().unwrap() - 1;
        let mut picked_up: Vec<char> = Vec::new();
        for _ in 0..instruction[1].parse::<usize>().unwrap(){
            let item: char = stacks[source].pop().unwrap();
            picked_up.push(item);
        }
        picked_up.reverse();
        stacks[destination].extend(picked_up.iter());
    }
    let mut output_string: String = String::new();
    for stack in stacks{
        output_string.push(stack[stack.len()-1]);
    }
    return output_string;
}