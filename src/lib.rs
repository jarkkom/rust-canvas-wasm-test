mod utils;
mod render;
mod math;

use wasm_bindgen::prelude::*;

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
    buffer: Vec<u8>,
    camera_pos: math::Vector3,
    camera_tar: math::Vector3,
    scene: render::scene::Scene,
    texture: render::Texture,
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
        let width = 1024;
        let height = 768;
        let buffer = vec![0; (width as usize)*(height as usize) * 4usize];
        let textdata = vec![0; (256 as usize)*(256 as usize) * 4usize];
        let scene = render::scene::Scene::new();
        CanvasRenderer {
            tick,
            width,
            height,
            buffer,
            camera_pos: math::Vector3{x: 0.0, y: 0.0, z: 1.0},
            camera_tar: math::Vector3{x: 0.0, y: 0.0, z: 0.0},
            scene,
            texture: render::Texture{ data: textdata, width: 256, height: 256 },
        }
    }

    pub fn set_camera_target(&mut self, x: f32, y: f32, z: f32) {
        self.camera_tar = math::Vector3{x: x, y: y, z: z};
    }

    pub fn set_camera_position(&mut self, x: f32, y: f32, z: f32) {
        self.camera_pos = math::Vector3{x: x, y: y, z: z};
    }

    pub fn set_texture(&mut self, data: Vec<u8>, width: i32, height: i32) {
        self.texture = render::Texture{data: data, width: width as u32, height: height as u32}
    }

    pub fn render(&mut self) {
        let target_buffer = self.buffer.clone();

        let mut current_target = render::RenderTarget{
            width: self.width(),
            height: self.height(),
            buffer: target_buffer,
            depth: std::vec::Vec::with_capacity((self.width() * self.height()) as usize), 
        };
        
        // zero
        current_target.depth.resize((self.width() * self.height()) as usize, 1.0);

        // zero
        for p in current_target.buffer.chunks_exact_mut(4) {
            p[0] = 0;
            p[1] = 0;
            p[2] = 0;
            p[3] = 255;
        }

        self.scene.camera.position = math::Vector3{ x: self.camera_pos.x, y: self.camera_pos.y, z: self.camera_pos.z };
        self.scene.camera.target = math::Vector3{ x: self.camera_tar.x, y: self.camera_tar.y, z: self.camera_tar.z };

        self.scene.draw(&mut current_target);

        {
            let i = (((self.height * self.width / 2 ) + self.width() / 2) * 4u32) as usize;
            current_target.buffer[i + 0] = 255;
            current_target.buffer[i + 1] = 255;
            current_target.buffer[i + 2] = 255;
            current_target.buffer[i + 3] = 255;
        }

        self.tick += 1;
        self.buffer = current_target.buffer;
    }

    pub fn add_obj(&mut self, obj_contents: String, texture_data: Vec<u8>, texture_width: i32, texture_height: i32) {
        let mut obj = render::scene::ObjLoader::load_obj(obj_contents);
        obj.texture = render::Texture{data: texture_data, width: texture_width as u32, height: texture_height as u32};

        self.scene.add_object(obj)
    }
}
