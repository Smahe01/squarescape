use crate::constants::*;
use crate::level::print_level;

use std::{thread, time::Duration};

/// When the player hit up arrow
pub fn up(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
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
pub fn down(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
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
pub fn left(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
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
pub fn right(v_rgb: &mut Vec<(u8, u8, u8)>, dimension: &(u32, u32), game_over: &mut bool, win: &mut bool){
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
