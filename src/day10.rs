use std::{fs::File, io::Read, isize, path::Path};

fn add_to_sum(cycle: usize, x: isize) -> isize {
    if cycle >= 20 {
        if (cycle - 20) % 40 == 0 || cycle == 20 {
            return x * (cycle as isize);
        }
    }
    return 0;
}

pub fn cathode_ray_tube_part_one() -> usize {
    let path: &Path = Path::new("day10.txt");
    let mut data_file: File = File::open(path).expect("Error opening file");
    let mut data: String = String::new();
    data_file
        .read_to_string(&mut data)
        .expect("Error reading from file");
    let mut cycle: usize = 0;
    let mut x: isize = 1;
    let mut sum_of_xs: isize = 0;
    for instruction in data.lines() {
        let processed_instruction: &[&str] = &instruction.split(" ").collect::<Vec<&str>>();
        cycle += 1;
        sum_of_xs += add_to_sum(cycle, x);
        if processed_instruction[0] == "addx" {
            cycle += 1;
            sum_of_xs += add_to_sum(cycle, x);
            x += processed_instruction[1]
                .parse::<isize>()
                .expect("Error parsing int");
        }
    }
    return sum_of_xs as usize;
}

fn add_to_display(output: &mut String, cycle: isize, x: isize){
    if (cycle)%40 == 0{
        output.push('\n');
    }
    if ((cycle%40)-x).abs() <= 1 {
        output.push('#');
    }else {
        output.push(' ');
    }
}

pub fn cathode_ray_tube_part_two() -> String {
    let path: &Path = Path::new("day10.txt");
    let mut data_file: File = File::open(path).expect("Error opening file");
    let mut data: String = String::new();
    data_file
        .read_to_string(&mut data)
        .expect("Error reading from file");
    let mut cycle: isize = 0;
    let mut x: isize = 1;
    let mut output: String = String::new();
    for instruction in data.lines() {
        let processed_instruction: &[&str] = &instruction.split(" ").collect::<Vec<&str>>();
        add_to_display(&mut output, cycle, x);
        cycle += 1;
        if processed_instruction[0] == "addx" {
            add_to_display(&mut output, cycle, x);
            cycle += 1;
            x += processed_instruction[1]
                .parse::<isize>()
                .expect("Error parsing int");
        }
    }
    return output;
}