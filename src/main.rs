mod constants;
pub use crate::constants::*;

use image::GenericImageView;
use std::process::Command;
use device_query::{DeviceQuery, DeviceState, Keycode};
use colored::*;
use std::{thread, time::Duration};

fn main() {
    // Disables the display of user input
    let mut no_print_input = Command::new("stty");
    no_print_input.arg("-echo");
    no_print_input.status().expect("failed to execute command");
    // Play the selected level
    let path_img = String::from("levels/1-Basics/1.png");
    play_level(&path_img);
    // Allow user input to be displayed (To reset to default)
    let mut print_input = Command::new("stty");
    print_input.arg("echo");
    print_input.status().expect("failed to execute command");
}

fn play_level(path_img: &String) {
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
fn print_level(v_rgb: &Vec<(u8, u8, u8)>, width: u32, length: u32){
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

/// When the player hit up arrow
fn up(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
    let width = usize::try_from(dimension.0).unwrap();
    loop {
        let index = v_rgb.iter().position(|&x| x == GREEN).unwrap();
        // Red
        if v_rgb[index - width] == RED{
            *game_over = true;
            break;
        }
        // White
        if v_rgb[index - width] == WHITE {
            v_rgb[index] = WHITE;
            v_rgb[index - width] = GREEN;
        }
        // Black
        if v_rgb[index - width] == BLACK{
            break;
        }
        // blue
        if v_rgb[index - width] == BLUE{
            *win = true;
            break;
        }
        thread::sleep(Duration::from_millis(REFRESH_TIME));
        print_level(&v_rgb, dimension.0, dimension.1);
    }
}

/// When the player hit down arrow
fn down(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
    let width = usize::try_from(dimension.0).unwrap();
    loop {
        let index = v_rgb.iter().position(|&x| x == GREEN).unwrap();
        // Red
        if v_rgb[index + width] == RED{
            *game_over = true;
            break;
        }
        // White
        if v_rgb[index + width] == WHITE {
            v_rgb[index] = WHITE;
            v_rgb[index + width] = GREEN;
        }
        // Black
        if v_rgb[index + width] == BLACK{
            break;
        }
        // blue
        if v_rgb[index + width] == BLUE{
            *win = true;
            break;
        }
        thread::sleep(Duration::from_millis(REFRESH_TIME));
        print_level(&v_rgb, dimension.0, dimension.1);
    }
}

/// When the player hit down arrow
fn left(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
    loop {
        let index = v_rgb.iter().position(|&x| x == GREEN).unwrap();
        // Red
        if v_rgb[index - 1] == RED{
            *game_over = true;
            break;
        }
        // White
        if v_rgb[index - 1] == WHITE {
            v_rgb[index] = WHITE;
            v_rgb[index - 1] = GREEN;
        }
        // Black
        if v_rgb[index - 1] == BLACK{
            break;
        }
        // blue
        if v_rgb[index - 1] == BLUE{
            *win = true;
            break;
        }
        thread::sleep(Duration::from_millis(REFRESH_TIME));
        print_level(&v_rgb, dimension.0, dimension.1);
    }
}

/// When the player hit down arrow
fn right(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
    loop {
        let index = v_rgb.iter().position(|&x| x == GREEN).unwrap();
        // Red
        if v_rgb[index + 1] == RED{
            *game_over = true;
            break;
        }
        // White
        if v_rgb[index + 1] == WHITE {
            v_rgb[index] = WHITE;
            v_rgb[index + 1] = GREEN;
        }
        // Black
        if v_rgb[index + 1] == BLACK{
            break;
        }
        // blue
        if v_rgb[index + 1] == BLUE{
            *win = true;
            break;
        }
        thread::sleep(Duration::from_millis(REFRESH_TIME));
        print_level(&v_rgb, dimension.0, dimension.1);
    }
}
