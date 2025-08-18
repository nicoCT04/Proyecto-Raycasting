// framebuffer.rs

use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn _render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        // 1) Cargar la textura en una variable aparte (salimos del if let)
        let texture = match window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            Ok(tex) => tex,
            Err(_) => return, // si falla, salimos silenciosamente
        };

        // 2) Lee FPS ANTES de begin_drawing (evita doble préstamo)
        let fps = window.get_fps();

        // 3) dibuja
        let mut renderer = window.begin_drawing(raylib_thread);
        // renderer.clear_background(Color::BLACK); // opcional
        renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        renderer.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::LIME);
        // renderer se libera aquí automáticamente
    }
}
