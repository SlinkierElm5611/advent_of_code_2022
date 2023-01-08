use std::{fs::File, io::Read, path::Path};

pub fn tuning_trouble_part_one() -> u32 {
    let path: &Path = Path::new("day6.txt");
    let mut file: File = match File::open(path) {
        Err(e) => panic!("Could not open file: {}", e),
        Ok(file) => file,
    };

    let mut raw_message: String = String::new();
    match file.read_to_string(&mut raw_message) {
        Err(e) => panic!("Error reading file data: {}", { e }),
        Ok(lenght) => lenght,
    };
    let len_of_start_packet: usize = 4;
    for i in 0..(raw_message.len() - len_of_start_packet) {
        let possible_packet: &str = &raw_message[i..(i + len_of_start_packet)];
        let mut start: bool = true;
        let mut already_seen: Vec<char> = Vec::new();
        for j in possible_packet.chars() {
            if !already_seen.contains(&j) {
                already_seen.push(j);
            } else {
                start = false;
            }
        }
        if start {
            return (i + len_of_start_packet) as u32;
        }
    }
    return raw_message.len() as u32;
}

pub fn tuning_trouble_part_two() -> u32 {
    let path: &Path = Path::new("day6.txt");
    let mut file: File = match File::open(path) {
        Err(e) => panic!("Could not open file: {}", e),
        Ok(file) => file,
    };

    let mut raw_message: String = String::new();
    match file.read_to_string(&mut raw_message) {
        Err(e) => panic!("Error reading file data: {}", { e }),
        Ok(lenght) => lenght,
    };
    let len_of_start_packet: usize = 14;
    for i in 0..(raw_message.len() - len_of_start_packet) {
        let possible_packet: &str = &raw_message[i..(i + len_of_start_packet)];
        let mut start: bool = true;
        let mut already_seen: Vec<char> = Vec::new();
        for j in possible_packet.chars() {
            if !already_seen.contains(&j) {
                already_seen.push(j);
            } else {
                start = false;
            }
        }
        if start {
            return (i + len_of_start_packet) as u32;
        }
    }
    return raw_message.len() as u32;
}
