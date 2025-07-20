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
    let _grid = Grid::new(window_width, window_height);

    while !window.window_should_close() {
        framebuffer.clear();

        framebuffer.set_current_color(Color::BLACK);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
