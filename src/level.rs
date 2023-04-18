use crate::constants::*;
use crate::direction::*;

use image::GenericImageView;
use std::process::Command;
use device_query::{DeviceQuery, DeviceState, Keycode};
use colored::*;

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
        if game_over {
            clear_screen();
            println!("You lost !");
            break;
        }
        if win {
            clear_screen();
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
            // let square = String::from("■");
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
