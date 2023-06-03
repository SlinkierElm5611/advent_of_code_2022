#[derive(Debug, Clone, Copy)]
struct Position {
    x: u16,
    y: u16,
}
#[derive(Debug)]
struct Rock {
    occupied_locations: [Position; 5],
}
impl Rock {
    fn build(rock_type: u8, current_max: u16) -> Rock {
        let mut new_rock: Rock = Rock {
            occupied_locations: [Position {
                x: u16::MIN,
                y: u16::MIN,
            }; 5],
        };
        match rock_type {
            0 => {
                for i in 0..4 {
                    new_rock.occupied_locations[i].x = 3 + (i as u16);
                    new_rock.occupied_locations[i].y = current_max + 4;
                }
            }
            1 => {
                for i in 0..3 {
                    new_rock.occupied_locations[i].x = 3 + (i as u16);
                    new_rock.occupied_locations[i].y = current_max + 5;
                }
                new_rock.occupied_locations[3].x = 4;
                new_rock.occupied_locations[3].y = current_max + 4;
                new_rock.occupied_locations[4].x = 4;
                new_rock.occupied_locations[4].y = current_max + 6;
            }
            2 => {
                for i in 0..3 {
                    new_rock.occupied_locations[i].x = 3 + (i as u16);
                    new_rock.occupied_locations[i].y = current_max + 4;
                }
                new_rock.occupied_locations[3].x = 4;
                new_rock.occupied_locations[3].y = current_max + 5;
                new_rock.occupied_locations[4].x = 4;
                new_rock.occupied_locations[4].y = current_max + 6;
            }
            3 => {
                for i in 0..4 {
                    new_rock.occupied_locations[i].x = 2;
                    new_rock.occupied_locations[i].y = current_max + 3 + (i as u16);
                }
            }
            4 => {
                for i in 0..2 {
                    for j in 0..2 {
                        new_rock.occupied_locations[(i * 2) + j].x = 2 + (i as u16);
                        new_rock.occupied_locations[(i * 2) + j].y = current_max + 3 + (j as u16);
                    }
                }
            }
            _ => panic!("Error! Rock Type Not Supported"),
        }
        return new_rock;
    }
}
#[derive(Debug)]
struct Cave {
    rocks: [Position; 8897],
    last_index: usize,
    current_rock_type: u8,
    current_highest: u16,
    air_currents: Vec<bool>,
}
impl Cave {
    fn build(air_currents_input: &str) -> Cave {
        let mut new_cave = Cave {
            rocks: [Position {
                x: u16::MAX,
                y: u16::MAX,
            }; 8897],
            last_index: usize::MIN,
            current_rock_type: u8::MIN,
            current_highest: u16::MIN,
            air_currents: vec![false; air_currents_input.len()],
        };
        for (index, char) in air_currents_input.chars().enumerate() {
            if char == '>' {
                new_cave.air_currents[index] = true;
            } else {
                new_cave.air_currents[index] = false;
            }
        }
        return new_cave;
    }
    fn simulate_rock(&mut self) {
        let mut new_rock: Rock = Rock::build(self.current_rock_type, self.last_index as u16);
        for i in 0..5 {
            if new_rock.occupied_locations[i].y != u16::MIN {
                self.rocks[self.last_index + i] = new_rock.occupied_locations[i];
            }
        }
        self.current_rock_type += 1;
        if self.current_rock_type == 5 {
            self.current_rock_type = 0;
        }
    }
    fn run_simulation(&mut self) {
        for _ in 0..2022 {
            self.simulate_rock();
        }
    }
}
pub fn pyroclastic_flow_part_one() -> u16 {
    let input: &str = &std::fs::read_to_string("day17.txt").expect("Error! Unable to read file");
    for i in 0..5 {
        let rock: Rock = Rock::build(i, 0);
        dbg!(rock);
    }
    let mut cave: Cave = Cave::build(input);
    dbg!(&cave);
    cave.run_simulation();
    return cave.current_highest;
}
