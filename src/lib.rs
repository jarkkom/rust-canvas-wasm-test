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
    cameraPos: math::Vector3,
    cameraTar: math::Vector3,
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
            cameraPos: math::Vector3{x: 0.0, y: 0.0, z: 1.0},
            cameraTar: math::Vector3{x: 0.0, y: 0.0, z: 0.0},
            scene,
            texture: render::Texture{ data: textdata, width: 256, height: 256 },
        }
    }

    pub fn setCameraTarget(&mut self, x: f32, y: f32, z: f32) {
        self.cameraTar = math::Vector3{x: x, y: y, z: z};
    }

    pub fn setCameraPosition(&mut self, x: f32, y: f32, z: f32) {
        self.cameraPos = math::Vector3{x: x, y: y, z: z};
    }

    pub fn setTexture(&mut self, data: Vec<u8>, width: i32, height: i32) {
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
        current_target.depth.resize((self.width() * self.height()) as usize, std::f32::NEG_INFINITY);

        let mut texture = render::Texture {
            width: 256,
            height: 256,
            data: std::vec::Vec::with_capacity((self.width() * self.height()) as usize * 4), 
        };

        texture.data.resize((self.width() * self.height()) as usize * 4, 0);
        for y in 0..256u32 {
            for x in 0..256u32 {
                texture.data[(y * 256 + x) as usize * 4 + 0] = x as u8;
                texture.data[(y * 256 + x) as usize * 4 + 1] = y as u8;
                texture.data[(y * 256 + x) as usize * 4 + 2] = (x + y) as u8;
            }
        }

        // zero

        for p in current_target.buffer.chunks_exact_mut(4) {
            p[0] = 0;
            p[1] = 0;
            p[2] = 0;
            p[3] = 255;
        }

        let m = math::Matrix4::identity();
        let c = math::Matrix4::lookat(self.cameraPos, self.cameraTar);
        let cr = math::Matrix4::lookat_rot(self.cameraPos, self.cameraTar);
        let p = math::Matrix4::projection(60.0 / 180.0 * std::f32::consts::PI, self.width() as f32 / self.height() as f32, 1.0, 10000.0);
        let f = m.multiply(c).multiply(p);

        //log!("{:?}", f);

        let fw = self.width() as f32;
        let fh = self.height() as f32;

        for obj in self.scene.objects.iter() {
            let mut tvs: Vec<math::Vector4> = Vec::with_capacity(obj.vertices.len());

            for vertex in obj.vertices.iter() {
                let tv = vertex.multiply(f);
                tvs.push(tv);
            }

            let mut tvns: Vec<math::Vector4> = Vec::with_capacity(obj.vertex_normals.len());
            for vnormal in obj.vertex_normals.iter() {
                let tv = vnormal.multiply(cr);
                tvns.push(tv);
            }

            let mut i = 0;
            for face in obj.faces.iter() {
                i += 1;

                let v1 = tvs[face.v0 as usize];
                let v2 = tvs[face.v1 as usize];
                let v3 = tvs[face.v2 as usize];

                //log!("v {:?}, {:?}, {:?} v1.z {:?}", v1, v2, v3, v1.z / v1.w);

                let ax1 = v3.sub(v1);
                let ax2 = v2.sub(v1);
                let cp = ax2.normal().cross(ax1.normal());

                //log!("cp {:?}", cp);
                // if cp.z > 0.0 {
                //     continue;
                // }

                if v1.z < 0.0 || v2.z < 0.0 || v3.z < 0.0  {
                    continue;
                }

                if v1.x.abs() > v1.w.abs() && v1.y.abs() > v1.w.abs()
                    && v2.x.abs() > v2.w.abs() && v2.y.abs() > v2.w.abs()
                    && v3.x.abs() > v3.w.abs() && v3.y.abs() > v3.w.abs() {
                    continue;
                }

                let x1 = (v1.x / v1.w) * (fw / 2.0) + (fw / 2.0);  
                let y1 = (v1.y / v1.w) * (fh / 2.0) + (fh / 2.0);  

                let x2 = (v2.x / v2.w) * (fw / 2.0) + (fw / 2.0);  
                let y2 = (v2.y / v2.w) * (fh / 2.0) + (fh / 2.0);  

                let x3 = (v3.x / v3.w) * (fw / 2.0) + (fw / 2.0);  
                let y3 = (v3.y / v3.w) * (fh / 2.0) + (fh / 2.0);  

                // let mut c = render::Color { r: (cp.z.abs() * 255.0) as u8, b: (cp.z.abs() * 0.0) as u8, g: (cp.z.abs() * 0.0) as u8, a: 255 };
                // if i % 2 == 1 
                // { 
                //     c = render::Color { r: (cp.z.abs() * 0.0) as u8, b: (cp.z.abs() * 255.0) as u8, g: (cp.z.abs() * 0.0) as u8, a: 255 }
                // }
                // render::draw_triangle_barycentric_z(&mut current_target,
                //     &c,
                //     math::Vector3{ x: x1, y: y1, z: v1.z / v1.w},
                //     math::Vector3{ x: x2, y: y2, z: v2.z / v2.w},
                //     math::Vector3{ x: x3, y: y3, z: v3.z / v3.w}
                //     );

                let euv0 = math::Point { x: (tvns[face.vn0 as usize].x / -2.0) + 0.5, y: (tvns[face.vn0 as usize].y / -2.0) + 0.5};
                let euv1 = math::Point { x: (tvns[face.vn1 as usize].x / -2.0) + 0.5, y: (tvns[face.vn1 as usize].y / -2.0) + 0.5};
                let euv2 = math::Point { x: (tvns[face.vn2 as usize].x / -2.0) + 0.5, y: (tvns[face.vn2 as usize].y / -2.0) + 0.5};

                // let euv0 = obj.uvs[face.uv0 as usize]
                // let euv0 = obj.uvs[face.uv1 as usize]
                // let euv0 = obj.uvs[face.uv2 as usize]

                render::draw_triangle_barycentric_z_uv(&mut current_target,
                    &self.texture,
                    math::Vector3{ x: x1, y: y1, z: v1.z / v1.w},
                    math::Vector3{ x: x2, y: y2, z: v2.z / v2.w},
                    math::Vector3{ x: x3, y: y3, z: v3.z / v3.w},
                    euv0,
                    euv1,
                    euv2,
                    );
            }
        }

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

    pub fn add_obj(&mut self, x: String) {
        let obj = render::scene::ObjLoader::load_obj(x);

        self.scene.add_object(obj)
    }
}
