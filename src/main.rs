use std::{thread, time::Duration};

use grid::Grid;
use raylib::prelude::*;
use framebuffer::FrameBuffer;

mod framebuffer;
mod grid;


fn main() {
    let window_width = 600;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Nils Muralles - Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = FrameBuffer::new(window_width, window_height, Color::WHITE);
    let grid_width = window_width / 10;
    let grid_height = window_height / 10;
    let mut grid = Grid::new(grid_width, grid_height, 10);

    // Crea un grid de pulsars
    let spacing = 20; // Espacio entre pulsars
    let pulsars_x = 3; // Cuántos en X
    let pulsars_y = 3; // Cuántos en Y

    for i in 0..pulsars_x {
        for j in 0..pulsars_y {
            let offset_x = i * spacing + 3;
            let offset_y = j * spacing + 3;
            framebuffer.draw_pulsar(&mut grid, offset_x, offset_y);
        }
    }

    while !window.window_should_close() {
        framebuffer.clear();
        framebuffer.set_background_color(Color::WHITE);
        framebuffer.set_current_color(Color::BLACK);

        grid.update();
        grid.draw(&mut framebuffer);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(60));
    }
}
