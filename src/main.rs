use image::GenericImageView;
use colored::*;

fn main() {
    let path_img = String::from("levels/1.png");
    let dimension : (u32,u32) = get_dimension(&path_img);
    let v_rgb : Vec<(u8, u8, u8)> = read_image(&path_img);
    print_level(v_rgb, dimension.0, dimension.1)
}

fn get_dimension(path_img: &String) -> (u32, u32) {
    let img = image::open(path_img).unwrap();
    return (img.dimensions().0, img.dimensions().1)
}

fn read_image(path_img: &String) ->  Vec<(u8, u8, u8)> {
    let img = image::open(path_img).unwrap();
    let mut v_rgb: Vec<(u8, u8, u8)> = Vec::new();
    for pixel in img.pixels() {
        v_rgb.push((pixel.2[0], pixel.2[1], pixel.2[2]))
    }
    return v_rgb
}

fn print_level(v_rgb: Vec<(u8, u8, u8)>, width: u32, length: u32){
    let mut nb_square : usize = 0;
    for _i in 0..length {
        for _j in 0..width {
            // let square = String::from("■");
            match v_rgb[nb_square]{
                (255, 0, 0) => print!("{}", "■".red()),
                (0, 255, 0) => print!("{}", "■".green()),
                (0, 0, 255) => print!("{}", "■".blue()),
                (0, 0, 0) => print!("■"),
                _ => print!(" "),
            } 
            // print!("■");
            nb_square += 1;
        }
        print!("\n")
    }
}
