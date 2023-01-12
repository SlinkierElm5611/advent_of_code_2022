struct Region {
    locations: std::collections::HashMap<(usize,usize), bool>,
    max_y: usize
}

impl Region {
    fn parse_rock(&mut self, rock: &str) {
        let mut previous_location: Option<(usize, usize)> = None;
        for location in rock.split("->"){
            let mut coordinate_iterator = location.trim().split(",").map(|item| item.parse::<usize>().expect("Error parsing int!"));
            let current_location: (usize, usize) = (coordinate_iterator.next().expect("No items left!"),coordinate_iterator.next().expect("No Items Left!"));
            if previous_location.is_some() {
                if current_location.0 == previous_location.as_ref().unwrap().0 {
                    for y in std::cmp::min(current_location.1, previous_location.as_ref().unwrap().1)..std::cmp::max(current_location.1, previous_location.as_ref().unwrap().1){
                        self.locations.insert((current_location.0, y), false);
                    }
                }
                if current_location.1 == previous_location.as_ref().unwrap().1 {
                    for x in std::cmp::min(current_location.0, previous_location.as_ref().unwrap().0)..std::cmp::max(current_location.0, previous_location.as_ref().unwrap().0){
                        self.locations.insert((x, current_location.1), false);
                    }
                }
            }
            previous_location = Some(current_location);
            self.locations.insert(current_location, false);
        }
    }
    fn parse(input: &str) -> Region{
        let mut new_region: Region = Region {locations: std::collections::HashMap::new(), max_y: usize::MIN};
        for rock in input.lines(){
            new_region.parse_rock(rock);
        }
        for key in new_region.locations.keys(){
            if key.1 > new_region.max_y{
                new_region.max_y = key.1;
            }
        }
        return new_region;
    }
    fn drop_unit_of_sand(&mut self) -> bool{
        let mut current_position: (usize,usize) = (500, 0);
        let mut settled: bool = false;
        loop {
            if !self.locations.contains_key(&(current_position.0, current_position.1+1)){
                current_position.1 += 1;
            }else if !self.locations.contains_key(&(current_position.0 -1, current_position.1+1)){
                current_position.1 += 1;
                current_position.0 -= 1;
            }else if !self.locations.contains_key(&(current_position.0 +1, current_position.1+1)){
                current_position.1 += 1;
                current_position.0 += 1;
            }else {
                settled = true;
                break;
            }
            if current_position.1 >= self.max_y {
                break;
            }
        }
        if settled {
            self.locations.insert(current_position, true);
        }
        return settled;
    }
    fn drop_unit_of_sand_two(&mut self) -> bool{
        let mut current_position: (usize,usize) = (500, 0);
        if self.locations.contains_key(&current_position){
            return false;
        }
        loop {
            if !self.locations.contains_key(&(current_position.0, current_position.1+1)) && current_position.1 < self.max_y +1{
                current_position.1 += 1;
            }else if !self.locations.contains_key(&(current_position.0 -1, current_position.1+1))&& current_position.1 < self.max_y +1{
                current_position.1 += 1;
                current_position.0 -= 1;
            }else if !self.locations.contains_key(&(current_position.0 +1, current_position.1+1))&& current_position.1 < self.max_y +1{
                current_position.1 += 1;
                current_position.0 += 1;
            }else {
                break;
            }
        }
        self.locations.insert(current_position, true);
        return true;
    }
    fn units_of_sand(&self) -> usize {
        let mut units: usize = 0;
        for &item in self.locations.values(){
            if item {
                units += 1;
            }
        }
        return units;
    }
}
pub fn rigolith_reservoir_part_one() -> usize {
    let input_string: String = std::fs::read_to_string("day14.txt").expect("Error reading from file!");
    let mut new_region: Region = Region::parse(&input_string);
    loop {
        if !new_region.drop_unit_of_sand() {
            break;
        }
    }
    return new_region.units_of_sand();
}
pub fn rigolith_reservoir_part_two() -> usize {
    let input_string: String = std::fs::read_to_string("day14.txt").expect("Error reading from file!");
    let mut new_region: Region = Region::parse(&input_string);
    loop {
        if !new_region.drop_unit_of_sand_two() {
            break;
        }
    }
    return new_region.units_of_sand();
}