mod constants;
mod direction;
mod level;

use crate::level::play_level;

use std::process::Command;


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
