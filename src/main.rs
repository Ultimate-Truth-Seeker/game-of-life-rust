mod line;
mod framebuffer;
mod gameoflife;
use std::{thread, time::Duration};

use crate::framebuffer::Framebuffer;
use gameoflife::GameOfLife;
use line::line;
use raylib::{color::Color, ffi::TraceLogLevel, math::Vector2};

fn main() {
    let w = 800;
    let h = 600;
    let cell_size = 1;

    let (mut window, raylib_thread) = raylib::init()
        .size(w, h)
        .title("Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();
    
    let mut fb = Framebuffer::new(w as u32, h as u32, Color::new(50, 50, 100, 255));
    let mut gl = GameOfLife::new(w as u32, h as u32, cell_size);
    let rle = include_str!("greyship.rle");
    let rle1 = include_str!("dart.rle");
    
    gl.load_rle(rle, 400, 100);
    gl.load_rle(rle1, 100, 600);
    fb.set_current_color(Color::WHITE);

    while !window.window_should_close() {        
        gl.update();
        fb.clear();
        gl.draw(&mut fb);
        fb.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
    }

}
