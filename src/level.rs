use crate::constants::*;
use crate::direction::*;

use image::GenericImageView;
use std::process::Command;
use device_query::{DeviceQuery, DeviceState, Keycode};
use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{thread, time::Duration};

pub fn play_level(path_img: &String) {
    let dimension: (u32,u32) = get_dimension(&path_img);
    let mut game_over: bool = false;
    let mut win: bool = false;
    let mut v_rgb: Vec<(u8, u8, u8)> = read_image(&path_img);
    let device_state = DeviceState::new();
    print_level(&v_rgb, dimension.0, dimension.1);
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::Up) {
            up(&mut v_rgb, &dimension, &mut game_over, &mut win);
        }
        if keys.contains(&Keycode::Down) {
            down(&mut v_rgb, &dimension, &mut game_over, &mut win);
        }
        if keys.contains(&Keycode::Left) {
            left(&mut v_rgb, &dimension, &mut game_over, &mut win);
        }
        if keys.contains(&Keycode::Right) {
            right(&mut v_rgb, &dimension, &mut game_over, &mut win);
        }
        if keys.contains(&Keycode::Q) {
            break;
        }
        if game_over {
            v_rgb = read_image(&path_img);
            thread::sleep(Duration::from_millis(DIED_TIME));
            print_level(&v_rgb, dimension.0, dimension.1);
            game_over = false;
        }
        if win {
            clear_screen();
            valide_level(&path_img);
            println!("Level passed !");
            break;
        }
        // println!("Is A pressed? {}", keys.contains(&Keycode::Up));
    }
}

/// Return dimension of the image
fn get_dimension(path_img: &String) -> (u32, u32) {
    let img = image::open(path_img).unwrap();
    return (img.dimensions().0, img.dimensions().1)
}

/// Return RGB of each pixels of the image in a vector
fn read_image(path_img: &String) ->  Vec<(u8, u8, u8)> {
    let img = image::open(path_img).unwrap();
    let mut v_rgb: Vec<(u8, u8, u8)> = Vec::new();
    for pixel in img.pixels() {
        v_rgb.push((pixel.2[0], pixel.2[1], pixel.2[2]))
    }
    return v_rgb
}

/// Clear the terminal
fn clear_screen() {
    let mut clear = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else {
        Command::new("clear")
    };
    clear.status().expect("failed to execute command");
}

/// Print the RGB vector in a terminal
pub fn print_level(v_rgb: &Vec<(u8, u8, u8)>, width: u32, length: u32){
    clear_screen();
    // Print the level
    let mut nb_square : usize = 0;
    for _i in 0..length {
        for _j in 0..width {
            match v_rgb[nb_square]{
                RED => print!("{}", "■".red()),
                GREEN => print!("{}", "■".green()),
                BLUE => print!("{}", "■".blue()),
                BLACK => print!("■"),
                _ => print!(" "),
            }
            // print!("■");
            nb_square += 1;
        }
        print!("\n")
    }
}


fn valide_level(path_img: &String) {
    let path_img_split = path_img.split("/");
    let mut path_img_vec: Vec<&str> = Vec::new();
    for i in path_img_split {
        path_img_vec.push(i);
    }

    // Read the file list.txt
    let filename = format!("{}/{}/list.txt", path_img_vec[0], path_img_vec[1]);
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(file);
    // vector that retrieves the content of each row of list.txt
    let mut v_list_level: Vec<String> = Vec::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        v_list_level.push(line);
    }

    // Change validate level
    let name_level_split = path_img_vec[2].split(".");
    let mut name_level: Vec<&str> = Vec::new();
    for i in name_level_split {
        name_level.push(i);
    }
    v_list_level[name_level[0].parse::<usize>().unwrap() - 1] = format!("OK {}", name_level[0]);

    // Write the file list.txt
    let mut file = File::create(&filename).unwrap();
    for i in v_list_level {
        writeln!(&mut file, "{}", i).unwrap();
    }

}
