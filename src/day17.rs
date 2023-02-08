#[derive(Debug)]
struct Rock {
    positions: Vec<(u16, u16)>,
    rock_type: u8,
}
impl Rock {
    fn generate_rock(rock_type: u8, current_highest_position: u16) -> Rock {
        match rock_type {
            0 => Rock {
                positions: vec![
                    (2, current_highest_position + 3),
                    (3, current_highest_position + 3),
                    (4, current_highest_position + 3),
                    (5, current_highest_position + 3),
                ],
                rock_type: rock_type,
            },
            1 => Rock {
                positions: vec![
                    (2, current_highest_position + 4),
                    (3, current_highest_position + 4),
                    (4, current_highest_position + 4),
                    (3, current_highest_position + 3),
                    (3, current_highest_position + 5),
                ],
                rock_type: rock_type,
            },
            2 => Rock {
                positions: vec![
                    (2, current_highest_position + 3),
                    (3, current_highest_position + 3),
                    (4, current_highest_position + 3),
                    (4, current_highest_position + 4),
                    (4, current_highest_position + 5),
                ],
                rock_type: rock_type,
            },
            3 => Rock {
                positions: vec![
                    (2, current_highest_position + 3),
                    (2, current_highest_position + 4),
                    (2, current_highest_position + 5),
                ],
                rock_type: rock_type,
            },
            4 => Rock {
                positions: vec![
                    (2, current_highest_position + 3),
                    (2, current_highest_position + 4),
                    (3, current_highest_position + 3),
                    (3, current_highest_position + 4),
                ],
                rock_type: rock_type,
            },
            _ => Rock {
                positions: vec![],
                rock_type: u8::MAX,
            },
        }
    }
}
#[derive(Debug)]
struct Cave {
    highest_occupied_position: u16,
    spawned_rocks: Vec<Rock>,
    currents: Vec<bool>
}
impl Cave {
    fn parse(input: &str) -> Cave {
        let mut new_cave: Cave = Cave { highest_occupied_position: 0, spawned_rocks: Vec::new(), currents: Vec::new() };
        for char in input.chars() {
            new_cave.currents.push(char=='>');
        }
        return new_cave;
    }
    fn spawn_rock(&mut self){
    }
}
pub fn pyroclastic_flow_part_one() -> u16 {
    let input: &str = &std::fs::read_to_string("day17.txt").expect("Error reading from file!");
    let mut new_cave: Cave = Cave::parse(input);
    for _ in 0..2022{
        new_cave.spawn_rock();
    }
    return new_cave.highest_occupied_position;
}
