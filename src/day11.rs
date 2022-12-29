use std::{fs::File, io::Read, path::Path};

#[derive(Debug)]
struct monkey {
    items_held: Vec<usize>,
    operation: [String; 3],
    test: usize,
    test_fail: usize,
    test_pass: usize,
    number_of_inspections: usize
}

pub fn monkey_in_the_middle_part_one() -> usize {
    let file_path: &Path = Path::new("day11.txt");
    let mut data_file: File = File::open(file_path).expect("Error opening file");
    let mut data_string: String = String::new();
    data_file
        .read_to_string(&mut data_string)
        .expect("Error reading file");
    let mut monkeys: Vec<monkey> = Vec::new();
    let mut lines: Vec<&str> = data_string.split("\n").collect();
    let mut index: usize = 0;
    while lines.contains(&"") {
        if lines[index] == "" {
            lines.remove(index);
        } else {
            index += 1;
        }
    }
    for i in 0..lines.len() / 6 {
        let mut chimp: monkey = monkey {
            items_held: Vec::new(),
            operation: [String::new(), String::new(), String::new()],
            test: lines[i*6 + 3].split(" ").collect::<Vec<&str>>().pop().expect("Improperly formatted input").parse().expect("Error parsing int"),
            test_fail: lines[i*6 + 5].split(" ").collect::<Vec<&str>>().pop().expect("Improperly formatted input").parse().expect("Error parsing int"),
            test_pass: lines[i*6 + 4].split(" ").collect::<Vec<&str>>().pop().expect("Improperly formatted input").parse().expect("Error parsing int"),
            number_of_inspections: 0
        };
        for item in lines[i * 6 + 1][lines[i * 6 + 1]
            .find(":")
            .expect("Improperly formatted input!")
            + 1..]
            .split(",")
            .map(|x| x.trim().parse::<usize>().expect("Error parsing int"))
        {
            chimp.items_held.push(item);
        }
        for (index, item) in lines[i * 6 + 2][lines[i * 6 + 2]
            .find("old")
            .expect("Improperly formatted input!")..]
            .split(" ")
            .enumerate()
        {
            chimp.operation[index].insert_str(0, item);
        }
        monkeys.push(chimp);
    }
    for _ in 0..20 {
        for chimp in 0..monkeys.len(){
            while monkeys[chimp].items_held.len() > 0{
                let mut item = monkeys[chimp].items_held.pop().expect("No items left to pop!");
                if monkeys[chimp].operation[2] == "old"{
                    if monkeys[chimp].operation[1] == "*"{
                        item *= item;
                    }else {
                        item += item;
                    }
                }else {
                    if monkeys[chimp].operation[1] == "*" {
                        item *= monkeys[chimp].operation[2].trim().parse::<usize>().expect("Error parsing int");
                    }else {
                        item += monkeys[chimp].operation[2].trim().parse::<usize>().expect("Error parsing int")
                    }
                }
                monkeys[chimp].number_of_inspections+=1;
                item = item/3;
                if item % monkeys[chimp].test == 0 {
                    let index = monkeys[chimp].test_pass;
                    monkeys[index].items_held.push(item);
                }else {
                    let index = monkeys[chimp].test_fail;
                    monkeys[index].items_held.push(item);                    
                }
            }
        }
    }
    let mut sorted_worries: Vec<usize> = monkeys.iter().map(|x| x.number_of_inspections).collect();
    sorted_worries.sort();
    return sorted_worries.pop().expect("No items left to pop")*sorted_worries.pop().expect("No items left to pop");
}

pub fn monkey_in_the_middle_part_two() -> usize {
    let file_path: &Path = Path::new("day11.txt");
    let mut data_file: File = File::open(file_path).expect("Error opening file");
    let mut data_string: String = String::new();
    data_file
        .read_to_string(&mut data_string)
        .expect("Error reading file");
    let mut monkeys: Vec<monkey> = Vec::new();
    let mut lines: Vec<&str> = data_string.split("\n").collect();
    let mut index: usize = 0;
    while lines.contains(&"") {
        if lines[index] == "" {
            lines.remove(index);
        } else {
            index += 1;
        }
    }
    for i in 0..lines.len() / 6 {
        let mut chimp: monkey = monkey {
            items_held: Vec::new(),
            operation: [String::new(), String::new(), String::new()],
            test: lines[i*6 + 3].split(" ").collect::<Vec<&str>>().pop().expect("Improperly formatted input").parse().expect("Error parsing int"),
            test_fail: lines[i*6 + 5].split(" ").collect::<Vec<&str>>().pop().expect("Improperly formatted input").parse().expect("Error parsing int"),
            test_pass: lines[i*6 + 4].split(" ").collect::<Vec<&str>>().pop().expect("Improperly formatted input").parse().expect("Error parsing int"),
            number_of_inspections: 0
        };
        for item in lines[i * 6 + 1][lines[i * 6 + 1]
            .find(":")
            .expect("Improperly formatted input!")
            + 1..]
            .split(",")
            .map(|x| x.trim().parse::<usize>().expect("Error parsing int"))
        {
            chimp.items_held.push(item);
        }
        for (index, item) in lines[i * 6 + 2][lines[i * 6 + 2]
            .find("old")
            .expect("Improperly formatted input!")..]
            .split(" ")
            .enumerate()
        {
            chimp.operation[index].insert_str(0, item);
        }
        monkeys.push(chimp);
    }
    let mut modulus: usize = 1;
    for chimp in &monkeys{
        modulus *= chimp.test;
    }
    for _ in 0..10_000 {
        for chimp in 0..monkeys.len(){
            while monkeys[chimp].items_held.len() > 0{
                let mut item = monkeys[chimp].items_held.pop().expect("No items left to pop!");
                if monkeys[chimp].operation[2] == "old"{
                    if monkeys[chimp].operation[1] == "*"{
                        item *= item;
                    }else {
                        item += item;
                    }
                }else {
                    if monkeys[chimp].operation[1] == "*" {
                        item *= monkeys[chimp].operation[2].trim().parse::<usize>().expect("Error parsing int");
                    }else {
                        item += monkeys[chimp].operation[2].trim().parse::<usize>().expect("Error parsing int")
                    }
                }
                monkeys[chimp].number_of_inspections+=1;
                item = item%modulus;
                if item % monkeys[chimp].test == 0 {
                    let index = monkeys[chimp].test_pass;
                    monkeys[index].items_held.push(item);
                }else {
                    let index = monkeys[chimp].test_fail;
                    monkeys[index].items_held.push(item);                    
                }
            }
        }
    }
    let mut sorted_worries: Vec<usize> = monkeys.iter().map(|x| x.number_of_inspections).collect();
    sorted_worries.sort();
    return sorted_worries.pop().expect("No items left to pop")*sorted_worries.pop().expect("No items left to pop");
}