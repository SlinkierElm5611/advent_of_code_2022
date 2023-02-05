#[derive(Debug)]
struct Valve<'a> {
    flowrate: u8,
    neighbours: std::collections::HashSet<&'a str>,
}
impl<'a> Valve<'a> {
    fn parse(input: &str) -> (&str, Valve) {
        let mut new_valve: Valve = Valve {
            flowrate: 0,
            neighbours: std::collections::HashSet::new(),
        };
        let mut name: &str = "";
        for (index, part) in input.split(" ").enumerate() {
            if index == 1 {
                name = part;
            } else if index == 4 {
                new_valve.flowrate = part[5..(part.len() - 1)]
                    .parse::<u8>()
                    .expect("Error parsing int");
            } else if index > 8 {
                new_valve.neighbours.insert(&part[0..2]);
            }
        }
        return (name, new_valve);
    }
}
#[derive(Debug, Clone)]
struct State<'a> {
    current_position: &'a str,
    open_valves: std::collections::HashMap<&'a str, u8>,
    time_elapsed: u8,
}
#[derive(Debug)]
struct Region<'a> {
    valves: std::collections::HashMap<&'a str, Valve<'a>>,
    flowing: std::collections::HashSet<&'a str>,
    start: &'a str,
}
impl<'a> Region<'a> {
    fn parse(input: &str) -> Region {
        let mut new_region: Region = Region {
            valves: std::collections::HashMap::new(),
            flowing: std::collections::HashSet::new(),
            start: &"AA",
        };
        for line in input.lines() {
            let result: (&str, Valve) = Valve::parse(line);
            if result.1.flowrate > 0 {
                new_region.flowing.insert(result.0);
            }
            new_region.valves.insert(result.0, result.1);
        }
        return new_region;
    }
    fn pop_min_unvisited_value(
        distances: &std::collections::HashMap<&str, u8>,
        unvisited: &mut std::collections::HashSet<&'a str>,
    ) -> &'a str {
        let mut unvisited_iter: std::collections::hash_set::Iter<&str> = unvisited.iter();
        let current: &str = unvisited_iter.next().expect("No items!");
        let mut current: (&str, &u8) = (current, distances.get(current).expect("No items!"));
        for location in unvisited_iter {
            let latest: &u8 = distances.get(location).expect("No such item!");
            if latest < current.1 {
                current.0 = location;
                current.1 = latest;
            }
        }
        unvisited.remove(current.0);
        return current.0;
    }
    fn get_distance_from_current_to_all_others(
        &self,
        current: &'a str,
    ) -> std::collections::HashMap<&str, u8> {
        let mut distances: std::collections::HashMap<&str, u8> = std::collections::HashMap::new();
        let mut unvisited: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for location in self.valves.keys() {
            unvisited.insert(location);
            distances.insert(location, u8::MAX);
        }
        distances.insert(current, u8::MIN);
        while !unvisited.is_empty() {
            let current_location: &str =
                Region::pop_min_unvisited_value(&distances, &mut unvisited);
            for neighbour in self
                .valves
                .get(current_location)
                .expect("No such item!")
                .neighbours
                .iter()
            {
                if *distances.get(neighbour).expect("No such item!")
                    > *distances.get(current_location).expect("No such item!") + 1
                {
                    distances.insert(
                        neighbour,
                        *distances.get(current_location).expect("No such item!") + 1,
                    );
                }
            }
        }
        return distances;
    }
    fn get_distances_between_each_flowing(&self) -> std::collections::HashMap<(&str, &str), u8> {
        let mut distances: std::collections::HashMap<(&str, &str), u8> =
            std::collections::HashMap::new();
        for valve in self.flowing.iter() {
            let distances_from_current: std::collections::HashMap<&str, u8> =
                self.get_distance_from_current_to_all_others(valve);
            for flowing in self.flowing.iter() {
                if flowing != valve {
                    distances.insert((valve, flowing), distances_from_current[flowing]);
                }
            }
        }
        return distances;
    }
    fn insert_start_position_to_distance_map(&self) -> std::collections::HashMap<(&str, &str), u8> {
        let mut distances: std::collections::HashMap<(&str, &str), u8> =
            self.get_distances_between_each_flowing();
        let distance_from_start: std::collections::HashMap<&str, u8> =
            self.get_distance_from_current_to_all_others(self.start);
        for valve in self.flowing.iter() {
            distances.insert(
                (self.start, valve),
                *distance_from_start.get(valve).unwrap(),
            );
        }
        return distances;
    }
    fn pressure_released(&self, state: &State) -> u16 {
        let mut pressure: u16 = u16::MIN;
        for valve in state.open_valves.keys() {
            let time_opened: u8 = state.open_valves[valve];
            if time_opened <= 30 {
                pressure += (30 - time_opened as u16) * (self.valves[valve].flowrate as u16);
            }
        }
        return pressure;
    }
    fn highest_pressure_released(&self) -> u16 {
        let mut states: std::collections::VecDeque<State> = std::collections::VecDeque::new();
        states.push_back(State {
            current_position: self.start,
            open_valves: std::collections::HashMap::new(),
            time_elapsed: 0,
        });
        let cost_between_positions: std::collections::HashMap<(&str, &str), u8> =
            self.insert_start_position_to_distance_map();
        let mut current_highest_pressure_release: u16 = u16::MIN;
        while !states.is_empty() {
            let current_state: State = states.pop_front().expect("No items left!");
            if current_state.time_elapsed >= 30
                || current_state.open_valves.len() == self.flowing.len()
            {
                let pressure_released: u16 = self.pressure_released(&current_state);
                if current_highest_pressure_release < pressure_released {
                    current_highest_pressure_release = pressure_released;
                }
                continue;
            }
            for unvisited_valve in self.flowing.iter() {
                if !current_state.open_valves.contains_key(unvisited_valve) {
                    let mut new_state: State = current_state.clone();
                    let cost: u8 =
                        cost_between_positions[&(new_state.current_position, *unvisited_valve)] + 1;
                    new_state.time_elapsed += cost;
                    new_state
                        .open_valves
                        .insert(unvisited_valve, new_state.time_elapsed);
                    new_state.current_position = unvisited_valve;
                    states.push_back(new_state);
                }
            }
        }
        return current_highest_pressure_release;
    }
    fn highest_pressure_released_with_elephant(&self) -> u16 {
        let mut max_release_per_configuration: std::collections::HashMap<
            std::collections::BTreeSet<&str>,
            u16,
        > = std::collections::HashMap::new();
        let mut states: std::collections::VecDeque<State> = std::collections::VecDeque::new();
        states.push_back(State {
            current_position: "AA",
            open_valves: std::collections::HashMap::new(),
            time_elapsed: 4,
        });
        let cost_between_positions: std::collections::HashMap<(&str, &str), u8> =
            self.insert_start_position_to_distance_map();
        while !states.is_empty() {
            let current_state: State = states.pop_front().expect("No items left!");
            let current_configuration: std::collections::BTreeSet<&str> = current_state.open_valves.keys().map(|string| *string).collect();
            let current_max: &u16 = max_release_per_configuration.get(&current_configuration).unwrap_or(&u16::MIN);
            let current_pressure_relieved: u16 = self.pressure_released(&current_state);
            if current_pressure_relieved > *current_max {
                max_release_per_configuration.insert(current_configuration, current_pressure_relieved);
            }
            if !(current_state.time_elapsed >= 30
                || current_state.open_valves.len() == self.flowing.len())
            {
                for unvisited_valve in self.flowing.iter() {
                    if !current_state.open_valves.contains_key(unvisited_valve) {
                        let mut new_state: State = current_state.clone();
                        let cost: u8 = cost_between_positions
                            [&(new_state.current_position, *unvisited_valve)]
                            + 1;
                        new_state.time_elapsed += cost;
                        new_state
                            .open_valves
                            .insert(unvisited_valve, new_state.time_elapsed);
                        new_state.current_position = unvisited_valve;
                        states.push_back(new_state);
                    }
                }
            }
        }
        let mut max_pressure_released: u16 = u16::MIN;
        for configuration_one in max_release_per_configuration.iter(){
            for configuration_two in max_release_per_configuration.iter(){
                if configuration_one!=configuration_two{
                    if configuration_one.0.intersection(configuration_two.0).count() == 0{
                        if configuration_one.1 + configuration_two.1 > max_pressure_released {
                            max_pressure_released = configuration_one.1+configuration_two.1;
                        }
                    }
                }
            }
        }
        return max_pressure_released;
    }
}
pub fn proboscidea_volcanium_part_one() -> u16 {
    let text: String = std::fs::read_to_string("day16.txt").expect("Error reading from file!");
    let region: Region = Region::parse(&text);
    return region.highest_pressure_released();
}
pub fn proboscidea_volcanium_part_two() -> u16 {
    let text: String = std::fs::read_to_string("day16.txt").expect("Error reading from file!");
    let region: Region = Region::parse(&text);
    return region.highest_pressure_released_with_elephant();
}
