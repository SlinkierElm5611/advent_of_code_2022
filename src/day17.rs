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
                x: u16::MAX,
                y: u16::MAX,
            }; 5],
        };
        match rock_type {
            0 => {
                for i in 0..4 {
                    new_rock.occupied_locations[i].x = 2 + (i as u16);
                    new_rock.occupied_locations[i].y = current_max + 3;
                }
            }
            1 => {
                for i in 0..3 {
                    new_rock.occupied_locations[i].x = 2 + (i as u16);
                    new_rock.occupied_locations[i].y = current_max + 4;
                }
                new_rock.occupied_locations[3].x = 3;
                new_rock.occupied_locations[3].y = current_max + 3;
                new_rock.occupied_locations[4].x = 3;
                new_rock.occupied_locations[4].y = current_max + 5;
            }
            2 => {
                for i in 0..3 {
                    new_rock.occupied_locations[i].x = 2 + (i as u16);
                    new_rock.occupied_locations[i].y = current_max + 3;
                }
                new_rock.occupied_locations[3].x = 4;
                new_rock.occupied_locations[3].y = current_max + 4;
                new_rock.occupied_locations[4].x = 4;
                new_rock.occupied_locations[4].y = current_max + 5;
            }
            3 => {
                for i in 0..4 {
                    new_rock.occupied_locations[i].x = 2;
                    new_rock.occupied_locations[i].y = current_max + 2 + (i as u16);
                }}
            4 => {
                for i in 0..2{
                    for j in 0..2{
                        new_rock.occupied_locations[(i*2)+j].x = 2 + (i as u16);
                        new_rock.occupied_locations[(i*2)+j].y = current_max + 3 + (j as u16);
                    }
                }
            }
            _ => panic!("Error! Rock Type Not Supported"),
        }
        return new_rock;
    }
}
#[derive(Debug)]
struct Cave {}
pub fn pyroclastic_flow_part_one() -> u16 {
    let input: &str = &std::fs::read_to_string("day17.txt").expect("Error reading from file!");
    for i in 0..5 {
        let rock: Rock = Rock::build(i, 0);
        dbg!(rock);
    }
    return 0;
}
