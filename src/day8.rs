use std::{io::Read};

fn on_edge(x: usize, y: usize, tree_map: &Vec<Vec<usize>>) -> bool {
    if x == 0 || x == &tree_map.len()-1 || y == 0 || y == &tree_map[x].len() - 1 {
        return true;
    }
    return false;
}

pub fn treetop_tree_house_part_one() -> usize {
    let path: &std::path::Path = std::path::Path::new("day8.txt");
    let mut file: std::fs::File = std::fs::File::open(path).expect("Error opening file");
    let mut input_data: String = String::new();
    let mut tree_heights: Vec<Vec<usize>> = Vec::new();
    file.read_to_string(&mut input_data)
        .expect("Error reading text from file");
    for line in input_data.lines() {
        let mut temp_vec: Vec<usize> = Vec::new();
        for index in 0..line.len() {
            temp_vec.push(line[index..=index].parse().unwrap())
        }
        tree_heights.push(temp_vec);
    }
    let mut trees_visible_from_outside: usize = 0;
    let mut tree_visibility: Vec<(usize, usize, [usize;4], bool)> = Vec::new();
    for x in 0..tree_heights.len() {
        for y in 0..tree_heights[x].len() {
            if on_edge(x, y, &tree_heights) {
                trees_visible_from_outside += 1;
            }else {
                let mut tree: (usize, usize, [usize;4], bool) = (x,y,[0;4], false);
                for i in 1..tree.0+1 {
                    let index: usize = x-i;
                    if tree_heights[index][y] >= tree_heights[x][y]{
                        tree.2[0] = i;
                        break;
                    }
                    if on_edge(index, y, &tree_heights){
                        tree.3 = true;
                    }
                }
                for i in 1..tree_heights.len()-tree.0{
                    let index: usize = x+i;
                    if tree_heights[index][y] >= tree_heights[x][y]{
                        tree.2[1] = i;
                        break;
                    }
                    if on_edge(index, y, &tree_heights){
                        tree.3 = true;
                    }
                }
                for j in 1..tree.1+1 {
                    let index: usize = y-j;
                    if tree_heights[x][index] >= tree_heights[x][y]{
                        tree.2[2] = j;
                        break;
                    }
                    if on_edge(x, index, &tree_heights){
                        tree.3 = true;
                    }
                }
                for j in 1..tree_heights[x].len()-tree.1{
                    let index: usize = y+j;
                    if tree_heights[x][index] >= tree_heights[x][y]{
                        tree.2[3] = j;
                        break;
                    }
                    if on_edge(x, index, &tree_heights){
                        tree.3 = true;
                    }
                }
                if tree.3 {
                    dbg!(tree);
                    trees_visible_from_outside += 1;
                    tree_visibility.push(tree);
                }
            }
        }
    }
    return trees_visible_from_outside;
}

pub fn treetop_tree_house_part_two() -> usize {
    let path: &std::path::Path = std::path::Path::new("day8.txt");
    let mut file: std::fs::File = std::fs::File::open(path).expect("Error opening file");
    let mut input_data: String = String::new();
    let mut tree_heights: Vec<Vec<usize>> = Vec::new();
    file.read_to_string(&mut input_data)
        .expect("Error reading text from file");
    for line in input_data.lines() {
        let mut temp_vec: Vec<usize> = Vec::new();
        for index in 0..line.len() {
            temp_vec.push(line[index..=index].parse().unwrap())
        }
        tree_heights.push(temp_vec);
    }
    let mut _trees_visible_from_outside: usize = 0;
    let mut tree_visibility: Vec<(usize, usize, [usize;4], bool)> = Vec::new();
    for x in 0..tree_heights.len() {
        for y in 0..tree_heights[x].len() {
            if on_edge(x, y, &tree_heights) {
                _trees_visible_from_outside += 1;
            }else {
                let mut tree: (usize, usize, [usize;4], bool) = (x,y,[0;4], false);
                for i in 1..tree.0+1 {
                    let index: usize = x-i;
                    if tree_heights[index][y] >= tree_heights[x][y]{
                        tree.2[0] = i;
                        break;
                    }
                    if on_edge(index, y, &tree_heights){
                        tree.2[0] = i;
                        tree.3 = true;
                    }
                }
                for i in 1..tree_heights.len()-tree.0{
                    let index: usize = x+i;
                    if tree_heights[index][y] >= tree_heights[x][y]{
                        tree.2[1] = i;
                        break;
                    }
                    if on_edge(index, y, &tree_heights){
                        tree.2[1] = i;
                        tree.3 = true;
                    }
                }
                for j in 1..tree.1+1 {
                    let index: usize = y-j;
                    if tree_heights[x][index] >= tree_heights[x][y]{
                        tree.2[2] = j;
                        break;
                    }
                    if on_edge(x, index, &tree_heights){
                        tree.2[2] = j;
                        tree.3 = true;
                    }
                }
                for j in 1..tree_heights[x].len()-tree.1{
                    let index: usize = y+j;
                    if tree_heights[x][index] >= tree_heights[x][y]{
                        tree.2[3] = j;
                        break;
                    }
                    if on_edge(x, index, &tree_heights){
                        tree.2[3] = j;
                        tree.3 = true;
                    }
                }
                if tree.3 {
                    _trees_visible_from_outside += 1;
                    tree_visibility.push(tree);
                }
            }
        }
    }
    let mut current_highest_scenic_score: usize = 0;
    for tree in tree_visibility{
        let mut scenic_score: usize = 1;
        for distance in tree.2{
            scenic_score *= distance;
        }
        if scenic_score > current_highest_scenic_score {
            current_highest_scenic_score = scenic_score;
        }
    }
    return current_highest_scenic_score;
}