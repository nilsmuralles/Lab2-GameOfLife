use raylib::prelude::*;

pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        return FrameBuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x > self.width || x < 0 {
           println!("Out of bounds on x"); 
        }
        if y > self.height || y < 0 {
           println!("Out of bounds on y"); 
        }
        self.color_buffer.draw_pixel(x, y, self.current_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }

    pub fn draw_block(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[(0, 0), (1, 0), (0, 1), (1, 1)] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_beehive(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_loaf(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_boat(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_tub(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[(1, 0), (0, 1), (2, 1), (1, 2)] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_blinker(&mut self, origin_x: i32, origin_y: i32) {
        for i in 0..3 {
            self.set_pixel(origin_x, origin_y + i);
        }
    }

    pub fn draw_toad(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[
            (1, 0), (2, 0), (3, 0),
            (0, 1), (1, 1), (2, 1),
        ] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_beacon(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[
            (0, 0), (1, 0), (0, 1), (1, 1),
            (2, 2), (3, 2), (2, 3), (3, 3),
        ] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_pulsar(&mut self, origin_x: i32, origin_y: i32) {
        let offsets = [
            // Top arms
            (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
            (0, 2), (5, 2), (7, 2), (12, 2),
            (0, 3), (5, 3), (7, 3), (12, 3),
            (0, 4), (5, 4), (7, 4), (12, 4),

            // Bottom arms (mirror)
            (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
            (0, 8), (5, 8), (7, 8), (12, 8),
            (0, 9), (5, 9), (7, 9), (12, 9),
            (0, 10), (5, 10), (7, 10), (12, 10),

            // Left arms
            (2, 2), (2, 3), (2, 4), (2, 8), (2, 9), (2, 10),
            (3, 5), (4, 5), (3, 7), (4, 7),
            (8, 5), (9, 5), (8, 7), (9, 7),
            (10, 2), (10, 3), (10, 4), (10, 8), (10, 9), (10, 10),
        ];
        for &(dx, dy) in &offsets {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_pentadecathlon(&mut self, origin_x: i32, origin_y: i32) {
        for i in 0..10 {
            self.set_pixel(origin_x, origin_y + i);
        }
        self.set_pixel(origin_x - 1, origin_y + 2);
        self.set_pixel(origin_x + 1, origin_y + 2);
        self.set_pixel(origin_x - 1, origin_y + 7);
        self.set_pixel(origin_x + 1, origin_y + 7);
    }

    pub fn draw_glider(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_lightweight_spaceship(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[
            (1, 0), (2, 0), (3, 0), (4, 0),
            (0, 1), (4, 1),
            (4, 2),
            (0, 3), (3, 3)
        ] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_middleweight_spaceship(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[
            (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
            (0, 1), (5, 1),
            (5, 2),
            (0, 3), (4, 3)
        ] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }

    pub fn draw_heavyweight_spaceship(&mut self, origin_x: i32, origin_y: i32) {
        for &(dx, dy) in &[
            (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0),
            (0, 1), (6, 1),
            (6, 2),
            (0, 3), (5, 3), (2, 3)
        ] {
            self.set_pixel(origin_x + dx, origin_y + dy);
        }
    }
}

