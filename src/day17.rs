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
                    new_rock.occupied_locations[i].x = 3 + (i as u16);
                    new_rock.occupied_locations[i].y = current_max + 3;
                }
            }
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            _ => panic!("Error! Rock Type Not Supported"),
        }
        return new_rock;
    }
}
#[derive(Debug)]
struct Cave {}
pub fn pyroclastic_flow_part_one() -> u16 {
    let input: &str = &std::fs::read_to_string("day17.txt").expect("Error reading from file!");
    return 0;
}
