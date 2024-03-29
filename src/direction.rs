use crate::constants::*;
use crate::level::print_level;

use std::{thread, time::Duration};

/// When the player hit up arrow
pub fn up(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
    let width = usize::try_from(dimension.0).unwrap();
    let mut index = v_rgb.iter().position(|&x| x == GREEN).unwrap();
    let first_index = index;
    let mut next_index: usize;
    loop {
        if index < width {
            next_index = v_rgb.len() - (width - index);
        } else {
            next_index = index - width;
        }
        // White or Green
        if v_rgb[next_index] == WHITE || v_rgb[next_index] == GREEN {
            v_rgb[index] = WHITE;
            v_rgb[next_index] = GREEN;
            index = next_index;
            // Infinite loop
            if index == first_index {
                thread::sleep(Duration::from_millis(REFRESH_TIME));
                print_level(&v_rgb, dimension.0, dimension.1);
                break;
            }
        } else {
            // Black
            if v_rgb[next_index] == BLACK{
                break;
            }
            // Red
            if v_rgb[next_index] == RED{
                *game_over = true;
                break;
            }
            // blue
            if v_rgb[next_index] == BLUE{
                *win = true;
                break;
            }
        }
        thread::sleep(Duration::from_millis(REFRESH_TIME));
        print_level(&v_rgb, dimension.0, dimension.1);
    }
}

/// When the player hit down arrow
pub fn down(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
    let width = usize::try_from(dimension.0).unwrap();
    let mut index = v_rgb.iter().position(|&x| x == GREEN).unwrap();
    let first_index = index;
    let mut next_index: usize;
    loop {
        if index + width >= v_rgb.len() {
            next_index = width - (v_rgb.len() - index);
        } else {
            next_index = index + width;
        }
        // White or Green
        if v_rgb[next_index] == WHITE || v_rgb[next_index] == GREEN {
            v_rgb[index] = WHITE;
            v_rgb[next_index] = GREEN;
            index = next_index;
            // Infinite loop
            if index == first_index {
                thread::sleep(Duration::from_millis(REFRESH_TIME));
                print_level(&v_rgb, dimension.0, dimension.1);
                break;
            }
        } else {
            // Black
            if v_rgb[next_index] == BLACK{
                break;
            }
            // Red
            if v_rgb[next_index] == RED{
                *game_over = true;
                break;
            }
            // blue
            if v_rgb[next_index] == BLUE{
                *win = true;
                break;
            }
        }

        thread::sleep(Duration::from_millis(REFRESH_TIME));
        print_level(&v_rgb, dimension.0, dimension.1);
    }
}

/// When the player hit down arrow
pub fn left(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
    let width = usize::try_from(dimension.0).unwrap();
    let mut index = v_rgb.iter().position(|&x| x == GREEN).unwrap();
    let first_index = index;
    let mut next_index: usize;
    loop {
        if index % width == 0 {
            next_index = index + width - 1;
        } else {
            next_index = index - 1;
        }
        // White or Green
        if v_rgb[next_index] == WHITE || v_rgb[next_index] == GREEN  {
            v_rgb[index] = WHITE;
            v_rgb[next_index] = GREEN;
            index = next_index;
            // Infinite loop
            if index == first_index {
                thread::sleep(Duration::from_millis(REFRESH_TIME));
                print_level(&v_rgb, dimension.0, dimension.1);
                break;
            }
        } else {
            // Black
            if v_rgb[next_index] == BLACK{
                break;
            }
            // Red
            if v_rgb[next_index] == RED{
                *game_over = true;
                break;
            }
            // blue
            if v_rgb[next_index] == BLUE{
                *win = true;
                break;
            }
        }
        thread::sleep(Duration::from_millis(REFRESH_TIME));
        print_level(&v_rgb, dimension.0, dimension.1);
    }
}

/// When the player hit down arrow
pub fn right(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
    let width = usize::try_from(dimension.0).unwrap();
    let mut index = v_rgb.iter().position(|&x| x == GREEN).unwrap();
    let first_index = index;
    let mut next_index: usize;
    loop {
        if index % width == width - 1 {
            next_index = 1 + index - width;
        } else {
            next_index = index + 1;
        }
        // White or Green
        if v_rgb[next_index] == WHITE || v_rgb[next_index] == GREEN {
            v_rgb[index] = WHITE;
            v_rgb[next_index] = GREEN;
            index = next_index;
            // Infinite loop
            if index == first_index {
                thread::sleep(Duration::from_millis(REFRESH_TIME));
                print_level(&v_rgb, dimension.0, dimension.1);
                break;
            }
        } else {
            // Black
            if v_rgb[next_index] == BLACK{
                break;
            }
            // Red
            if v_rgb[next_index] == RED{
                *game_over = true;
                break;
            }
            // blue
            if v_rgb[next_index] == BLUE{
                *win = true;
                break;
            }
        }
        thread::sleep(Duration::from_millis(REFRESH_TIME));
        print_level(&v_rgb, dimension.0, dimension.1);
    }
}
