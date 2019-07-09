extern crate piston_window;

use piston_window::*;

const WINDOW_HEIGHT: u32 = 640;
const WINDOW_WIDTH: u32 = 480;


fn main() {

    let game_title = "This is the game title";

    let mut window: PistonWindow =
        WindowSettings::new(game_title, [WINDOW_HEIGHT, WINDOW_WIDTH])
            .exit_on_esc(true)
            .build()
            .unwrap();

    while let Some(event) = window.next() {

        window.draw_2d(&event, |context, graphics, _device| {

            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);

        });

    }

}