#[derive(Debug, Clone)]
struct Input {
    num: Option<usize>,
    list: Option<Vec<Input>>,
}
impl Input {
    fn parse(line: &[char]) -> (Input, usize) {
        let mut new_vec: Vec<Input> = Vec::new();
        let mut counter: usize = 1;
        let mut working_string: String = String::new();
        loop {
            if counter >= line.len() {
                break;
            }
            if line[counter] == ',' && line[counter - 1] == '[' {
                new_vec.push(Input {
                    num: Some(5),
                    list: None,
                })
            }
            if line[counter] == ',' || line[counter] == ']' {
                if !working_string.is_empty() {
                    new_vec.push(Input {
                        num: Some(working_string.parse::<usize>().expect("Error Parsing Int")),
                        list: None,
                    });
                    working_string.clear();
                }
            } else {
                if line[counter] != '[' {
                    working_string.push(line[counter]);
                }
            }
            if line[counter] == '[' {
                let (new_input, distance) = Input::parse(&line[counter..]);
                new_vec.push(new_input);
                counter += distance;
            } else if line[counter] == ']' {
                break;
            }
            counter += 1;
        }
        return (
            Input {
                num: None,
                list: Some(new_vec),
            },
            counter,
        );
    }
    fn parse_pairs(data_input: &str) -> Vec<(Input, Input)> {
        let mut pairs: Vec<(Input, Input)> = Vec::new();
        let mut working_memory: Vec<Input> = Vec::new();
        for line in data_input.lines() {
            if line == "" {
                working_memory.clear();
            } else {
                working_memory.push(Input::parse(line.chars().collect::<Vec<char>>().as_slice()).0);
            }
            if working_memory.len() == 2 {
                pairs.push((working_memory[0].clone(), working_memory[1].clone()));
            }
        }
        return pairs;
    }
    fn is_right_order(pair: &(Input, Input)) -> Option<bool> {
        if pair.0.num.is_some() {
            // first item in pair is an integer
            if pair.1.num.is_some() {
                // second item in pair is an integer
                if pair.0.num.unwrap() == pair.1.num.unwrap() {
                    return None;
                } else if pair.0.num.unwrap() < pair.1.num.unwrap() {
                    return Some(true);
                } else {
                    return Some(false);
                }
            } else {
                // second item in pair is a list
                if pair.1.list.as_ref().unwrap().is_empty() {
                    return Some(false);
                } else {
                    return match Input::is_right_order(&(
                        pair.0.clone(),
                        pair.1.list.as_ref().unwrap()[0].clone(),
                    )) {
                        None => Some(true),
                        Some(value) => Some(value),
                    };
                }
            }
        } else {
            // first item in pair is a list
            if pair.1.num.is_some() {
                // second item in pair is an integer
                if pair.0.list.as_ref().unwrap().is_empty() {
                    return Some(true);
                } else {
                    return match Input::is_right_order(&(
                        pair.0.list.as_ref().unwrap()[0].clone(),
                        pair.1.clone(),
                    )) {
                        None => Some(true),
                        Some(value) => Some(value),
                    };
                }
            } else {
                // second item in pair is a list
                for i in 0..std::cmp::min(
                    pair.0.list.as_ref().unwrap().len(),
                    pair.1.list.as_ref().unwrap().len(),
                ) {
                    let check: Option<bool> = Input::is_right_order(&(
                        pair.0.list.as_ref().unwrap()[i].clone(),
                        pair.1.list.as_ref().unwrap()[i].clone(),
                    ));
                    if check != None {
                        return check;
                    }
                }
                if pair.0.list.as_ref().unwrap().len() == pair.1.list.as_ref().unwrap().len() {
                    return None;
                } else if pair.0.list.as_ref().unwrap().len() > pair.1.list.as_ref().unwrap().len()
                {
                    return Some(false);
                } else {
                    return Some(true);
                }
            }
        }
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        match Input::is_right_order(&(self.clone(), other.clone())) {
            None => true,
            Some(_) => false,
        }
    }
}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match Input::is_right_order(&(self.clone(), other.clone())) {
            None => Some(std::cmp::Ordering::Equal),
            Some(value) => match value {
                true => Some(std::cmp::Ordering::Less),
                false => Some(std::cmp::Ordering::Greater),
            },
        }
    }
}

impl Eq for Input {}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match Input::is_right_order(&(self.clone(), other.clone())) {
            None => std::cmp::Ordering::Equal,
            Some(value) => match value {
                true => std::cmp::Ordering::Less,
                false => std::cmp::Ordering::Greater,
            },
        }
    }
}

pub fn distress_signal_part_one() -> usize {
    let input_text: String = std::fs::read_to_string("day13.txt").expect("Error reading file!");
    let pairs: Vec<(Input, Input)> = Input::parse_pairs(&input_text);
    let mut total: usize = 0;
    for (index, pair) in pairs.iter().enumerate() {
        if Input::is_right_order(pair).unwrap() {
            total += index + 1;
        }
    }
    return total;
}

pub fn distress_signal_part_two() -> usize {
    let input_text: String = std::fs::read_to_string("day13.txt").expect("Error reading file!");
    let mut inputs: Vec<Input> = Vec::new();
    for line in input_text.lines() {
        if line != "" {
            inputs.push(Input::parse(&line.chars().collect::<Vec<char>>()).0);
        }
    }

    inputs.push(Input::parse(&"[[2]]".chars().collect::<Vec<char>>()).0);
    inputs.push(Input::parse(&"[[6]]".chars().collect::<Vec<char>>()).0);
    inputs.sort();
    let mut mutliple: usize = 1;
    for (index, input) in inputs.into_iter().enumerate() {
        if input == Input::parse(&"[[2]]".chars().collect::<Vec<char>>()).0 || input == Input::parse(&"[[6]]".chars().collect::<Vec<char>>()).0{
            mutliple *= index+1;
        }
    }
    return mutliple;
}
