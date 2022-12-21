use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

pub fn rock_paper_scissors_part_one() -> u32 {
    let path = Path::new("day2.txt");
    let mut file: File = match File::open(&path) {
        Err(_) => panic!("could not open file!"),
        Ok(file) => file,
    };
    let mut raw_games = String::new();
    match file.read_to_string(&mut raw_games) {
        Err(e) => panic!("Error reading from file {}", { e }),
        Ok(lenght) => lenght,
    };
    let game_vec: Vec<&str> = raw_games.split("\n").collect();
    let mut points: u32 = 0;
    let mut win: HashMap<&str, &str> = HashMap::new();
    win.insert("A", "C");
    win.insert("B", "A");
    win.insert("C", "B");
    let mut lose: HashMap<&str, &str> = HashMap::new();
    lose.insert("A", "B");
    lose.insert("B", "C");
    lose.insert("C", "A");
    for game in game_vec {
        let mut moves: Vec<&str> = game.split(" ").collect();
        if moves[1] == "X"{
            moves[1] = "A";
            points += 1;
        }else if moves[1] == "Y"{
            moves[1] = "B";
            points += 2;
        }else if moves[1] == "Z" {
            moves[1] = "C";
            points += 3;
        }
        if win[moves[1]] == moves[0] {
            points += 6;
        }else if moves[1] == moves[0] {
            points += 3;
        }
    }
    return points;
}

pub fn rock_paper_scissors_part_two() -> u32 {
    let path = Path::new("day2.txt");
    let mut file: File = match File::open(&path) {
        Err(_) => panic!("could not open file!"),
        Ok(file) => file,
    };
    let mut raw_games = String::new();
    match file.read_to_string(&mut raw_games) {
        Err(e) => panic!("Error reading from file {}", { e }),
        Ok(lenght) => lenght,
    };
    let game_vec: Vec<&str> = raw_games.split("\n").collect();
    let mut points: u32 = 0;
    let mut win: HashMap<&str, &str> = HashMap::new();
    win.insert("A", "C");
    win.insert("B", "A");
    win.insert("C", "B");
    let mut lose: HashMap<&str, &str> = HashMap::new();
    lose.insert("C", "A");
    lose.insert("A", "B");
    lose.insert("B", "C");
    for game in game_vec {
        let mut moves: Vec<&str> = game.split(" ").collect();
        if moves[1] == "X"{
            moves[1] = win[moves[0]];
        }else if moves[1] == "Y"{
            moves[1] = moves[0];
        }else{
            moves[1] = lose[moves[0]];
        }
        if moves[1] == "A"{
            points += 1;
        }else if moves[1] == "B"{
            points += 2;
        }else if moves[1] == "C" {
            points += 3;
        }
        if win[moves[1]] == moves[0] {
            points += 6;
        }else if moves[1] == moves[0] {
            points += 3;
        }
    }
    return points;
}