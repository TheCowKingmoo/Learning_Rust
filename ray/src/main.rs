extern crate minifb;
extern crate quicksilver;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

use minifb::{Key, WindowOptions, Window, Menu};

use quicksilver::{
    Result,
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Image},
    lifecycle::{Window, Event,},
};

use quicksilver::input::*;


fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut menu = Menu::new( "name").unwrap();

    window.set_title("This is a Window");
    let menu_handle = window.add_menu( &menu );
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}