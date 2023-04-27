mod constants;
mod direction;
mod menu;

mod level;

use crate::menu::{title_screen, main_menu};
use crate::level::play_level;

use std::process::Command;


fn main() {
    // Disables the display of user input
    let mut no_print_input = Command::new("stty");
    no_print_input.arg("-echo");
    no_print_input.status().expect("failed to execute command");
    // Open the Menu
    let _unused = title_screen();
    loop {
        let path_img = main_menu();
        match path_img.as_str() {
            "exit" => break,
            "back_in_menu" => {},
            _ => play_level(&path_img)
        }
    }
    // Allow user input to be displayed (To reset to default)
    let mut print_input = Command::new("stty");
    print_input.arg("echo");
    print_input.status().expect("failed to execute command");
}
