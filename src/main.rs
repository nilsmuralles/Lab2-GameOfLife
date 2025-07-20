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

    let offset_x = 1;
    let offset_y = 1;

    let gun = [
        (1, 5), (1, 6), (2, 5), (2, 6),
        (11, 5), (11, 6), (11, 7),
        (12, 4), (12, 8),
        (13, 3), (13, 9),
        (14, 3), (14, 9),
        (15, 6),
        (16, 4), (16, 8),
        (17, 5), (17, 6), (17, 7),
        (18, 6),
        (21, 3), (21, 4), (21, 5),
        (22, 3), (22, 4), (22, 5),
        (23, 2), (23, 6),
        (25, 1), (25, 2), (25, 6), (25, 7),
        (35, 3), (35, 4),
        (36, 3), (36, 4),
    ];

    for &(x, y) in &gun {
        grid.add_cell(offset_x + x, offset_y + y);
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
