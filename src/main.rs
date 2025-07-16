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

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("window example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height, Color::WHITE);

    framebuffer.set_current_color(Color::GREEN);
    line(&mut framebuffer, &Vertex::new(50, 50), &Vertex::new(350, 350));

    framebuffer.set_current_color(Color::RED);
    line(&mut framebuffer, &Vertex::new(350, 50), &Vertex::new(50, 350));

    while !window.window_should_close() {
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
