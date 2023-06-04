#[derive(Debug, Clone, Copy)]
enum Direction {
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u16,
    y: u16,
}
impl Position {
    fn is_valid(&self) -> bool {
        return self.x < 8 && self.x != u16::MIN && self.y != u16::MIN;
    }
}
#[derive(Debug)]
struct Rock {
    occupied_locations: [Position; 5],
    lenght: u8,
}
impl Rock {
    fn build(rock_type: u8, current_max: u16) -> Rock {
        let mut new_rock: Rock = Rock {
            occupied_locations: [Position {
                x: u16::MIN,
                y: u16::MIN,
            }; 5],
            lenght: u8::MIN,
        };
        match rock_type {
            0 => {
                for i in 0..4 {
                    new_rock.occupied_locations[i].x = 3 + (i as u16);
                    new_rock.occupied_locations[i].y = current_max + 4;
                }
                new_rock.lenght = 4;
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
                new_rock.lenght = 5;
            }
            2 => {
                for i in 0..3 {
                    new_rock.occupied_locations[i].x = 3 + (i as u16);
                    new_rock.occupied_locations[i].y = current_max + 4;
                }
                new_rock.occupied_locations[3].x = 5;
                new_rock.occupied_locations[3].y = current_max + 5;
                new_rock.occupied_locations[4].x = 5;
                new_rock.occupied_locations[4].y = current_max + 6;
                new_rock.lenght = 5;
            }
            3 => {
                for i in 0..4 {
                    new_rock.occupied_locations[i].x = 3;
                    new_rock.occupied_locations[i].y = current_max + 4 + (i as u16);
                }
                new_rock.lenght = 4;
            }
            4 => {
                for i in 0..2 {
                    for j in 0..2 {
                        new_rock.occupied_locations[(i * 2) + j].x = 3 + (i as u16);
                        new_rock.occupied_locations[(i * 2) + j].y = current_max + 4 + (j as u16);
                    }
                }
                new_rock.lenght = 4;
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
    air_currents: Vec<Direction>,
    current_index: usize,
}
impl Cave {
    fn parse_input(air_currents_input: &str) -> Vec<Direction> {
        let mut air_currents: Vec<Direction> = Vec::new();
        for char in air_currents_input.chars() {
            if char == '>' {
                air_currents.push(Direction::Right);
            } else if char == '<' {
                air_currents.push(Direction::Left);
            }
        }
        return air_currents;
    }
    fn build(air_currents_input: &str) -> Cave {
        let parsed_air_currents: Vec<Direction> = Cave::parse_input(air_currents_input);
        let new_cave = Cave {
            rocks: [Position {
                x: u16::MIN,
                y: u16::MIN,
            }; 8897],
            last_index: usize::MIN,
            current_rock_type: u8::MIN,
            current_highest: u16::MIN,
            air_currents: parsed_air_currents,
            current_index: usize::MIN,
        };
        return new_cave;
    }
    fn set_current_highest(&mut self) {
        let mut current_highest: u16 = u16::MIN;
        for i in 0..8897 {
            if self.rocks[i].is_valid() && self.rocks[i].y > current_highest {
                current_highest = self.rocks[i].y;
            }
        }
        self.current_highest = current_highest;
    }
    fn is_position_taken(&self, position_to_check: &Position) -> bool {
        for i in 0..8897 {
            if position_to_check == &self.rocks[i] {
                return true;
            }
        }
        return false;
    }
    fn move_rock(&self, rock: &mut Rock, direction: &Direction) -> bool {
        let mut new_location: [Position; 5] = [Position {
            x: u16::MIN,
            y: u16::MIN,
        }; 5];
        let mut is_move_allowed: bool = true;
        match direction {
            Direction::Down => {
                for i in 0..5 {
                    if rock.occupied_locations[i].is_valid() {
                        new_location[i] = Position {
                            x: rock.occupied_locations[i].x,
                            y: rock.occupied_locations[i].y - 1,
                        };
                    }
                }
            }
            Direction::Left => {
                for i in 0..5 {
                    if rock.occupied_locations[i].is_valid() {
                        new_location[i] = Position {
                            x: rock.occupied_locations[i].x - 1,
                            y: rock.occupied_locations[i].y,
                        };
                    }
                }
            }
            Direction::Right => {
                for i in 0..5 {
                    if rock.occupied_locations[i].is_valid() {
                        new_location[i] = Position {
                            x: rock.occupied_locations[i].x + 1,
                            y: rock.occupied_locations[i].y,
                        };
                    }
                }
            }
        }
        for i in 0..5 {
            if (!new_location[i].is_valid() || self.is_position_taken(&new_location[i]))
                && i < rock.lenght as usize
            {
                is_move_allowed = false;
            }
        }
        if is_move_allowed {
            for i in 0..5 {
                rock.occupied_locations[i] = new_location[i];
            }
        }
        return is_move_allowed;
    }
    fn simulate_rock(&mut self) {
        let mut new_rock: Rock = Rock::build(self.current_rock_type, self.current_highest);
        loop {
            self.move_rock(&mut new_rock, &self.air_currents[self.current_index]);
            self.current_index += 1;
            if self.current_index == self.air_currents.len() {
                self.current_index = 0;
            }
            if !self.move_rock(&mut new_rock, &Direction::Down) {
                break;
            }
        }
        for i in 0..5 {
            if new_rock.occupied_locations[i].is_valid() {
                self.rocks[self.last_index + i] = new_rock.occupied_locations[i];
            }
        }
        self.set_current_highest();
        self.last_index += new_rock.lenght as usize;
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
    let mut cave: Cave = Cave::build(input);
    cave.run_simulation();
    return cave.current_highest;
}
