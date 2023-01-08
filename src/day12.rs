use std::collections::hash_set::Iter;

#[derive(Debug)]
struct Region {
    start_position: (usize, usize),
    end_position: (usize, usize),
    map: Vec<Vec<u8>>,
}

impl Region {
    fn parse_map(raw_input: &str) -> Region {
        let mut new_region: Region = Region {
            start_position: (0, 0),
            end_position: (0, 0),
            map: Vec::new(),
        };
        let mut conversion_table: std::collections::HashMap<char, u8> =
            std::collections::HashMap::new();
        for (index, item) in "abcdefghijklmnopqrstuvwxyz".chars().enumerate() {
            conversion_table.insert(item, index as u8);
        }
        conversion_table.insert('S', 0);
        conversion_table.insert('E', 25);
        for (x, line) in raw_input.lines().enumerate() {
            let mut new_vec: Vec<u8> = Vec::new();
            for (y, location) in line.chars().enumerate() {
                new_vec.push(
                    *conversion_table
                        .get(&location)
                        .expect("Error! unexpected input!"),
                );
                if location == 'S' {
                    new_region.start_position = (x, y);
                } else if location == 'E' {
                    new_region.end_position = (x, y);
                }
            }
            new_region.map.push(new_vec);
        }
        return new_region;
    }
    fn is_valid_position(&self, position: (usize, usize)) -> bool {
        if position.0 < self.map.len() && position.1 < self.map[position.0].len() {
            return true;
        }
        return false;
    }
    fn get_neighbours(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbour_vec: Vec<(usize, usize)> = Vec::new();
        if self.is_valid_position(position) {
            if position.0 > 0 {
                let temp_pos: (usize, usize) = (position.0 - 1, position.1);
                if self.map[position.0][position.1] <= self.map[temp_pos.0][temp_pos.1] + 1 {
                    neighbour_vec.push(temp_pos);
                }
            }
            if position.0 < self.map.len() - 1 {
                let temp_pos: (usize, usize) = (position.0 + 1, position.1);
                if self.map[position.0][position.1] <= self.map[temp_pos.0][temp_pos.1] + 1 {
                    neighbour_vec.push(temp_pos);
                }
            }
            if position.1 > 0 {
                let temp_pos: (usize, usize) = (position.0, position.1 - 1);
                if self.map[position.0][position.1] <= self.map[temp_pos.0][temp_pos.1] + 1 {
                    neighbour_vec.push(temp_pos);
                }
            }
            if position.1 < self.map[position.0].len() - 1 {
                let temp_pos: (usize, usize) = (position.0, position.1 + 1);
                if self.map[position.0][position.1] <= self.map[temp_pos.0][temp_pos.1] + 1 {
                    neighbour_vec.push(temp_pos);
                }
            }
        }
        return neighbour_vec;
    }
    fn get_minimum_unvisited_location(
        &self,
        dist: &Vec<Vec<usize>>,
        unvisited: &std::collections::HashSet<(usize, usize)>,
    ) -> (usize, usize) {
        let mut location_iterator: Iter<(usize, usize)> = unvisited.iter();
        let mut current_lowwest: &(usize, usize) = location_iterator
            .next()
            .expect("No items left in iterator!");
        for unvisited_location in location_iterator {
            if dist[current_lowwest.0][current_lowwest.1]
                > dist[unvisited_location.0][unvisited_location.1]
            {
                current_lowwest = unvisited_location;
            }
        }
        return *current_lowwest;
    }
    fn djistra(&self) -> Vec<Vec<usize>> {
        let mut distances: Vec<Vec<usize>> =
            vec![vec![usize::MAX; self.map[0].len()]; self.map.len()];
        let mut unvisited: std::collections::HashSet<(usize, usize)> =
            std::collections::HashSet::new();
        for j in 0..distances.len() {
            for i in 0..distances[j].len() {
                unvisited.insert((j, i));
            }
        }
        distances[self.end_position.0][self.end_position.1] = 0;
        let mut current_node: (usize, usize);
        loop {
            if unvisited.is_empty() {
                break;
            }
            current_node = self.get_minimum_unvisited_location(&distances, &unvisited);
            if distances[current_node.0][current_node.1] == usize::MAX {
                break;
            }
            unvisited.remove(&current_node);
            for neighbour in self.get_neighbours(current_node) {
                if distances[neighbour.0][neighbour.1]
                    > distances[current_node.0][current_node.1] + 1
                {
                    distances[neighbour.0][neighbour.1] =
                        distances[current_node.0][current_node.1] + 1;
                }
            }
        }
        return distances;
    }
}

pub fn hill_climbing_algorithm_part_one() -> usize {
    let data_input: String = std::fs::read_to_string("day12.txt").expect("Error reading file!");
    let region: Region = Region::parse_map(&data_input);
    return region.djistra()[region.start_position.0][region.start_position.1];
}

pub fn hill_climbing_algorithm_part_two() -> usize {
    let data_input: String = std::fs::read_to_string("day12.txt").expect("Error reading file!");
    let region: Region = Region::parse_map(&data_input);
    let distances = region.djistra();
    let mut current_lowest: usize = usize::MAX;
    for i in 0..distances.len() {
        for j in 0..distances[i].len() {
            if region.map[i][j] == 0 {
                if current_lowest > distances[i][j] {
                    current_lowest = distances[i][j];
                }
            }
        }
    }
    return current_lowest;
}
