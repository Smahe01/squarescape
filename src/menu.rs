use youchoose;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn main_menu() -> String {
    // Retrieve list of levels category
    let mut list_category = Command::new("ls");
    list_category.arg("levels");
    let category_command = list_category.output().expect("failed to execute command");
    let category_string = core::str::from_utf8(&category_command.stdout).unwrap();
    let mut category_char = category_string.chars();
    category_char.next_back();
    let category_string = category_char.as_str();
    let category_split = category_string.split("\n");
    let mut category_vec: Vec<&str> = Vec::new();
    for i in category_split {
        category_vec.push(i);
    }

    // Category Menu
    let mut category_menu = youchoose::Menu::new(category_vec.iter())
        .icon("-");
    let choice_category = category_menu.show(); // `choice` is a Vec<usize> containing the chosen indices

    if choice_category.len() == 0 {
        return "exit".to_string();
    }

    // Level Menu
    let filename = format!("levels/{}/list.txt", category_vec[choice_category[0]]);
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // vector that retrieves the content of each row of list.txt
    let mut v_list_level: Vec<String> = Vec::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        v_list_level.push(line);
    }

    let mut level_menu = youchoose::Menu::new(v_list_level.iter())
        .icon("");
    let choice_level = level_menu.show();

    if choice_level.len() == 0 {
        return "exit".to_string();
    }

    let choice_split = v_list_level[choice_level[0]].split(" ");
    let mut level_vec: Vec<&str> = Vec::new();
    for i in choice_split {
        level_vec.push(i);
    }
    return format!("levels/{}/{}.png", category_vec[choice_category[0]], level_vec[1]);
}
