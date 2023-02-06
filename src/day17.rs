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
struct Cave {}
pub fn pyroclastic_flow_part_one() -> u16 {
    return 0;
}
