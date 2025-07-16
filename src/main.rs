use std::{thread, time::Duration};

use line::line;
use raylib::prelude::*;
use framebuffer::FrameBuffer;
use vertex::Vertex;

mod framebuffer;
mod vertex;
mod line;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("window example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = FrameBuffer::new(window_width, window_height, Color::WHITE);

    let mut translate_x = 0;
    let mut translate_y = 0;

    while !window.window_should_close() {
        translate_x += 1;
        translate_y += 1;
        framebuffer.clear();

        framebuffer.set_current_color(Color::GREEN);
        line(&mut framebuffer, &Vertex::new(50 + translate_x, 50 + translate_y), &Vertex::new(350 + translate_x, 350 + translate_y));

        framebuffer.set_current_color(Color::RED);
        line(&mut framebuffer, &Vertex::new(350 + translate_x, 50 + translate_y), &Vertex::new(50 + translate_x, 350 + translate_y));

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
