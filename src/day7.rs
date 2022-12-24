use std::io::Read;

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    sub_dirs: Vec<Box<Directory>>,
    size: usize,
}

impl Directory {
    fn new(name: Option<String>) -> Directory {
        return Directory {
            name: name.unwrap_or(String::from("default")),
            files: Vec::new(),
            sub_dirs: Vec::new(),
            size: 0,
        };
    }
    fn generate_fs(&mut self, cmd_output: &[&str]) -> usize {
        let mut counter: usize = 0;
        while counter < cmd_output.len() {
            if &cmd_output[counter][0..1] == "$" {
                if cmd_output[counter][2..4] == *"cd" {
                    if cmd_output[counter][5..] == *".." {
                        return counter + 1;
                    }
                    let mut temp_index: usize = 0;
                    for (index, dir) in self.sub_dirs.iter().enumerate() {
                        if dir.name.as_str() == &cmd_output[counter][5..] {
                            temp_index = index;
                            break;
                        }
                    }
                    counter += self.sub_dirs[temp_index].generate_fs(&cmd_output[counter + 1..]);
                }
            } else {
                if &cmd_output[counter][..3] == "dir" {
                    let new_dir = Box::new(Directory::new(Some(String::from(
                        &cmd_output[counter][4..],
                    ))));
                    self.sub_dirs.push(new_dir);
                } else {
                    let space_index: usize =
                        cmd_output[counter].find(" ").expect("error in line 47");
                    self.files.push(File {
                        size: cmd_output[counter][..space_index]
                            .parse()
                            .expect("Error parsing on line 48"),
                    })
                }
            }
            counter += 1;
        }
        return counter;
    }
    fn set_size(&mut self) -> usize {
        if self.size == 0 {
            let mut working_size: usize = 0;
            for item in &self.files {
                working_size += item.size;
            }
            self.size = working_size;
            for dir in &mut self.sub_dirs {
                working_size += dir.set_size();
            }
            self.size = working_size;
        }
        return self.size;
    }
}

fn sum_of_sizes_below_100000(dir: &Box<Directory>) -> usize {
    let mut working_size: usize = 0;
    if dir.size <= 100000 {
        working_size += dir.size;
    }
    for item in &dir.sub_dirs {
        working_size += sum_of_sizes_below_100000(item);
    }
    return working_size;
}

fn smallest_dir_to_delete(dir: &Box<Directory>, space_to_be_freed: usize) -> Option<usize> {
    let mut working_value: usize;
    if dir.size > space_to_be_freed {
        working_value = dir.size;
        for sub_dir in &dir.sub_dirs {
            let temp: usize =
                smallest_dir_to_delete(&sub_dir, space_to_be_freed).unwrap_or(usize::MAX);
            if working_value > temp {
                working_value = temp;
            }
        }
        return Some(working_value);
    }
    return None;
}

pub fn file_system_part_one() -> usize {
    let path: &std::path::Path = std::path::Path::new("day7.txt");
    let mut data_file: std::fs::File = match std::fs::File::open(path) {
        Err(e) => panic!("Error opening file!: {}", { e }),
        Ok(file) => file,
    };
    let mut input_string: String = String::new();
    data_file
        .read_to_string(&mut input_string)
        .expect("Error reading from file");
    let terminal_output: Vec<&str> = input_string.split("\n").collect();
    let mut root_dir: Box<Directory> = Box::new(Directory::new(Some(String::from("/"))));
    root_dir.generate_fs(&terminal_output[1..]);
    root_dir.set_size();
    return sum_of_sizes_below_100000(&root_dir);
}

pub fn file_system_part_two() -> usize {
    let path: &std::path::Path = std::path::Path::new("day7.txt");
    let mut data_file: std::fs::File = match std::fs::File::open(path) {
        Err(e) => panic!("Error opening file!: {}", { e }),
        Ok(file) => file,
    };
    let mut input_string: String = String::new();
    data_file
        .read_to_string(&mut input_string)
        .expect("Error reading from file");
    let terminal_output: Vec<&str> = input_string.split("\n").collect();
    let mut root_dir: Box<Directory> = Box::new(Directory::new(Some(String::from("/"))));
    let total_space_requried: usize = 30_000_000;
    root_dir.generate_fs(&terminal_output[1..]);
    let space_available: usize = 70_000_000 - root_dir.set_size();
    let space_to_be_freed: usize = total_space_requried - space_available;
    dbg!(space_to_be_freed);
    return smallest_dir_to_delete(&root_dir, space_to_be_freed).unwrap_or(0);
}
