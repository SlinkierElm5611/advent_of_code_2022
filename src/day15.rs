#[derive(Debug)]
struct Tunnels {
    pairs: std::collections::HashMap<(isize, isize), (isize, isize)>,
}
impl Tunnels {
    fn parse_sensor(&mut self, sensor: &str) {
        let sensor_data: Vec<&str> = sensor.split(" ").collect::<Vec<&str>>();
        self.pairs.insert(
            (
                sensor_data[2][2..sensor_data[2].len() - 1]
                    .parse::<isize>()
                    .expect("Error parsing int"),
                sensor_data[3][2..sensor_data[3].len() - 1]
                    .parse::<isize>()
                    .expect("Error parsing int"),
            ),
            (
                sensor_data[8][2..sensor_data[8].len() - 1]
                    .parse::<isize>()
                    .expect("Error parsing int"),
                sensor_data[9][2..]
                    .parse::<isize>()
                    .expect("Error parsing int"),
            ),
        );
    }
    fn parse(sensor_input: &str) -> Tunnels {
        let mut new_tunnels: Tunnels = Tunnels {
            pairs: std::collections::HashMap::new(),
        };
        for line in sensor_input.lines() {
            new_tunnels.parse_sensor(line);
        }
        return new_tunnels;
    }
    fn get_manhattan_distance(start: (isize, isize), end: (isize, isize)) -> isize {
        return (start.0 - end.0).abs() + (start.1 - end.1).abs();
    }
    fn num_positions_empty(&self, row: isize) -> usize {
        let mut set_of_positions_not_available: std::collections::HashSet<(isize, isize)> =
            std::collections::HashSet::new();
        for key in self.pairs.keys() {
            let manhattan_distance: isize = Tunnels::get_manhattan_distance(*key, self.pairs[key]);
            if (key.1 - row).abs() <= manhattan_distance {
                set_of_positions_not_available.insert((key.0, row));
                for position in 1..=(manhattan_distance - (key.1 - row).abs()) {
                    set_of_positions_not_available.insert((key.0 + position, row));
                    set_of_positions_not_available.insert((key.0 - position, row));
                }
            }
            set_of_positions_not_available.remove(key);
            set_of_positions_not_available
                .remove(self.pairs.get(key).expect("No such value for given key!"));
        }
        return set_of_positions_not_available.len();
    }
    fn location_of_undetected_beacon(&self, max_location: isize) -> (isize, isize) {
        let mut valid_locations: std::collections::HashSet<(isize, isize)> =
            std::collections::HashSet::new();
        let mut current_location: Option<(isize, isize)> = None;
        let mut manhattan_distances: std::collections::HashMap<(isize, isize), isize> =
            std::collections::HashMap::new();
        for key in self.pairs.keys() {
            manhattan_distances
                .insert(*key, Tunnels::get_manhattan_distance(self.pairs[key], *key));
        }
        for center in self.pairs.keys() {
            let edge: isize = manhattan_distances[center] + 1;
            for counter in 0..=edge {
                let locations: [(isize, isize); 4] = [
                    (center.0 + counter, center.1 + counter - edge),
                    (center.0 + counter, center.1 + edge - counter),
                    (center.0 - counter, center.1 + counter - edge),
                    (center.0 - counter, center.1 + edge - counter),
                ];
                let mut is_valid: [bool; 4] = [true; 4];
                for location in manhattan_distances.keys() {
                    for i in 0..4 {
                        if Tunnels::get_manhattan_distance(locations[i], *location) <= manhattan_distances[location]{
                            is_valid[i] = false;
                        }
                    }
                }
                for i in 0..4 {
                    if is_valid[i] {
                        valid_locations.insert(locations[i]);
                    }
                }
            }
        }
        for location in valid_locations {
            if location.0 >= 0 && location.0 <= max_location {
                if location.1 >= 0 && location.1 <= max_location {
                    current_location = Some(location);
                }
            }
        }
        return current_location.unwrap_or((0, 0));
    }
}
pub fn beacon_exclusion_zone_part_one() -> usize {
    let new_tunnels: Tunnels =
        Tunnels::parse(&std::fs::read_to_string("day15.txt").expect("Error reading from file!"));
    return new_tunnels.num_positions_empty(2000000);
}
pub fn beacon_exclusion_zone_part_two() -> usize {
    let new_tunnels: Tunnels =
        Tunnels::parse(&std::fs::read_to_string("day15.txt").expect("Error reading from file!"));
    let location: (isize, isize) = new_tunnels.location_of_undetected_beacon(4000000);
    return (location.0 * 4000000) as usize + location.1 as usize;
}
