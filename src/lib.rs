mod utils;

use wasm_bindgen::prelude::*;
use std::mem;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct CanvasRenderer {
    tick: u32,
    width: u32,
    height: u32,
    buffer: Vec<u8>
}

#[wasm_bindgen]
impl CanvasRenderer {

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn buffer(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn new() -> CanvasRenderer {
        utils::set_panic_hook();
        let tick = 0u32;
        let width = 320u32;
        let height = 240u32;
        let buffer = vec![0; (width as usize)*(height as usize) * 4usize];
        CanvasRenderer {
            tick,
            width,
            height,
            buffer
        }
    }

    fn drap_triangle_top(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let dx1 = (x2 - x1) / (y2 - y1);
        let dx2 = (x3 - x1) / (y3 - y1);

        let mut cx1 = x1;
        let mut cx2 = x1;

        for y in y1 as u32..y2 as u32 {
            let mut sx = cx1 as u32;
            let mut ex = cx2 as u32;
            if ex < sx {
                mem::swap(&mut sx, &mut ex)
            }

            for x in sx..ex {
                let i = (((y * self.width) + sx) * 4u32) as usize;
                self.buffer[i + 0] = 255;
                self.buffer[i + 1] = 0;
                self.buffer[i + 2] = 0;
                self.buffer[i + 3] = 255;

                cx1 += dx1;
                cx2 += dx2;
            }
        }
    }

    fn draw_triangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let mut hi_x = x1;
        let mut hi_y = y1;        
        let mut mi_x = x2;
        let mut mi_y = y2;        
        let mut lo_x = x3;
        let mut lo_y = y3;

        // sort vertices
        if hi_y > mi_y {
            mem::swap(&mut hi_x, &mut mi_x);
            mem::swap(&mut hi_y, &mut mi_y);
        }
        if hi_y > lo_y {
            mem::swap(&mut hi_x, &mut lo_x);
            mem::swap(&mut hi_y, &mut lo_y);
        }
        if mi_y > lo_y {
            mem::swap(&mut mi_x, &mut lo_x);
            mem::swap(&mut mi_y, &mut lo_y);
        }

        if mi_y == lo_y {

        } else if hi_y == mi_y {
            self.drap_triangle_top(x1, y1, x2, y2, x3, y3)
        } else {
            let x4 = x1;
        }
    }

    pub fn render(&mut self) {
        let mut target = self.buffer.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let i = (((y * self.width) + x) * 4u32) as usize;
                target[i + 0] = ((x + y + self.tick + 0) & 255) as u8;
                target[i + 1] = ((x + y + self.tick + 1) & 255) as u8;
                target[i + 2] = ((x + y + self.tick + 2) & 255) as u8;
                target[i + 3] = 255;
            } 
        }
        self.tick += 1;
        self.buffer = target
    }
}