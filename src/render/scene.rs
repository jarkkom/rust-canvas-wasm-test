use crate::math;

#[derive(Debug)]
pub struct Camera {
    pub position: math::Vector3,
    pub target: math::Vector3,
    pub field_of_vision: f32,
    pub aspect_ratio: f32,
}

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
}

#[derive(Debug)]
pub struct Face {
    pub v0: i32,
    pub v1: i32,
    pub v2: i32,
    pub vn0: i32,
    pub vn1: i32,
    pub vn2: i32,
    pub uv0: i32,
    pub uv1: i32,
    pub uv2: i32,
}

#[derive(Debug)]
pub struct Object {
    pub vertices: Vec<math::Vector4>,
    pub faces: Vec<Face>,
    pub vertex_normals: Vec<math::Vector4>,
    pub uvs: Vec<math::Point>,
    pub texture: super::Texture,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: math::Vector3 { x: 0.0, y: 0.0, z: 1.0 },
            target: math::Vector3 { x: 0.0, y: 0.0, z: 0.0},
            field_of_vision: 60.0,
            aspect_ratio: 3.0 / 2.0,
        }
    }
}

impl Scene {
    pub fn new() -> Scene {
        let objs: Vec<Object> = vec![];

        Scene { objects: objs, camera: Camera::new() }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }

    fn clip(v: Vec<&super::VertexUV>) -> Vec<super::VertexUV> {
        let mut z1_out:Vec<super::VertexUV> = Vec::with_capacity(v.len() + 1);

        // near-z
        for i in 0..v.len() {
            let next_i = (i + 1) % v.len();

            let dot = v[i].z + v[i].w;
            let dot_next = v[next_i].z + v[next_i].w;

            if dot >= 0.0 {
                z1_out.push(super::VertexUV{ x: v[i].x, y: v[i].y, z: v[i].z, w: v[i].w, u: v[i].u, v: v[i].v });
            }
            if dot < 0.0 && dot_next < 0.0 {
                continue;
            }
            if dot.signum() != dot_next.signum() {
                let a = (dot_next) / (dot_next - dot);
                z1_out.push(v[i].lerp(v[next_i], a));
            }
        }

        // far-z
        let mut z2_out:Vec<super::VertexUV> = Vec::with_capacity(z1_out.len() + 1);
        for i in 0..z1_out.len() {
            let next_i = (i + 1) % z1_out.len();

            let dot = -z1_out[i].z + z1_out[i].w;
            let dot_next = -z1_out[next_i].z + z1_out[next_i].w;

            if dot >= 0.0 {
                z2_out.push(super::VertexUV{ x: z1_out[i].x, y: z1_out[i].y, z: z1_out[i].z, w: z1_out[i].w, u: z1_out[i].u, v: z1_out[i].v });
            }
            if dot < 0.0 && dot_next < 0.0 {
                continue;
            }
            if dot.signum() != dot_next.signum() {
                let a = (dot_next) / (dot_next - dot);
                z2_out.push(z1_out[i].lerp(&z1_out[next_i], a));
            }
        }

        let mut x1_out:Vec<super::VertexUV> = Vec::with_capacity(z2_out.len() + 1);
        // x < w
        for i in 0..z2_out.len() {
            let next_i = (i + 1) % z2_out.len();

            let dot = z2_out[i].x + z2_out[i].w;
            let dot_next = z2_out[next_i].x + z2_out[next_i].w;

            if dot >= 0.0 {
                x1_out.push(super::VertexUV{ x: z2_out[i].x, y: z2_out[i].y, z: z2_out[i].z, w: z2_out[i].w, u: z2_out[i].u, v: z2_out[i].v });
            }
            if dot < 0.0 && dot_next < 0.0 {
                continue;
            }
            if dot.signum() != dot_next.signum() {
                let a = (dot_next) / (dot_next - dot);
                x1_out.push(z2_out[i].lerp(&z2_out[next_i], a));
            }
        }

        let mut x2_out:Vec<super::VertexUV> = Vec::with_capacity(x1_out.len() + 1);
        // x > -w
        for i in 0..x1_out.len() {
            let next_i = (i + 1) % x1_out.len();

            let dot = -x1_out[i].x + x1_out[i].w;
            let dot_next = -x1_out[next_i].x + x1_out[next_i].w;

            if dot >= 0.0 {
                x2_out.push(super::VertexUV{ x: x1_out[i].x, y: x1_out[i].y, z: x1_out[i].z, w: x1_out[i].w, u: x1_out[i].u, v: x1_out[i].v });
            }
            if dot < 0.0 && dot_next < 0.0 {
                continue;
            }
            if dot.signum() != dot_next.signum() {
                let a = (dot_next) / (dot_next - dot);
                x2_out.push(x1_out[i].lerp(&x1_out[next_i], a));
            }
        }
        return x2_out;
    }

    pub fn draw(&self, render_target: &mut super::RenderTarget) {

        let aspect_ratio = render_target.width as f32 / render_target.height as f32;

        let identity_matrix = math::Matrix4::identity();
        let view_matrix = math::Matrix4::lookat(&self.camera.position, &self.camera.target);
        let view_rotation_matrix = math::Matrix4::lookat_rot(&self.camera.position, &self.camera.target);
        let projection_matrix = math::Matrix4::projection(self.camera.field_of_vision / 180.0 * std::f32::consts::PI, aspect_ratio, 1.0, 1000.0);
        let final_matrix = identity_matrix.multiply(&view_matrix).multiply(&projection_matrix);

        let fw = render_target.width as f32;
        let fh = render_target.height as f32;

        for obj in self.objects.iter() {
            let mut transformed_vertices: Vec<math::Vector4> = Vec::with_capacity(obj.vertices.len());

            for vertex in obj.vertices.iter() {
                let tv = vertex.multiply(&final_matrix);
                transformed_vertices.push(tv);
            }

            let mut transformed_normals: Vec<math::Vector4> = Vec::with_capacity(obj.vertex_normals.len());
            for vnormal in obj.vertex_normals.iter() {
                let tv = vnormal.multiply(&view_rotation_matrix);
                transformed_normals.push(tv);
            }

            for face in obj.faces.iter() {
                let cv1 = &transformed_vertices[face.v0 as usize];
                let cv2 = &transformed_vertices[face.v1 as usize];
                let cv3 = &transformed_vertices[face.v2 as usize];

                // let ax1 = cv3.sub(&cv1);
                // let ax2 = cv2.sub(&cv1);
                // let cp = ax2.cross(&ax1);
                // if cp.z < 0.0 {
                //     continue;
                // }

                if cv1.x.abs() > cv1.w.abs() && cv1.y.abs() > cv1.w.abs()
                    && cv2.x.abs() > cv2.w.abs() && cv2.y.abs() > cv2.w.abs()
                    && cv3.x.abs() > cv3.w.abs() && cv3.y.abs() > cv3.w.abs() {
                    continue;
                }

                let euv0 = math::Point { x: (transformed_normals[face.vn0 as usize].x / -2.0) + 0.5, y: (transformed_normals[face.vn0 as usize].y / -2.0) + 0.5};
                let euv1 = math::Point { x: (transformed_normals[face.vn1 as usize].x / -2.0) + 0.5, y: (transformed_normals[face.vn1 as usize].y / -2.0) + 0.5};
                let euv2 = math::Point { x: (transformed_normals[face.vn2 as usize].x / -2.0) + 0.5, y: (transformed_normals[face.vn2 as usize].y / -2.0) + 0.5};

                // let euv0 = obj.uvs[face.uv0 as usize];
                // let euv1 = obj.uvs[face.uv1 as usize];
                // let euv2 = obj.uvs[face.uv2 as usize];

                let v1 = super::VertexUV{ x: cv1.x, y: cv1.y, z: cv1.z, w: cv1.w, u: euv0.x, v: euv0.y };
                let v2 = super::VertexUV{ x: cv2.x, y: cv2.y, z: cv2.z, w: cv2.w, u: euv1.x, v: euv1.y };
                let v3 = super::VertexUV{ x: cv3.x, y: cv3.y, z: cv3.z, w: cv3.w, u: euv2.x, v: euv2.y };

                let to_clip = vec![
                    &v1, 
                    &v2,
                    &v3,
                ];

                let clipped = self::Scene::clip(to_clip);

                if clipped.len() == 0 {
                    continue;
                }

                for i in 0..clipped.len() - 2 {
                    let v1 = &clipped[0];
                    let v2 = &clipped[i+1];
                    let v3 = &clipped[i+2];

                    let x1 = (v1.x / v1.w) * (fw / 2.0) + (fw / 2.0);  
                    let y1 = (v1.y / v1.w) * (fh / 2.0) + (fh / 2.0);  

                    let x2 = (v2.x / v2.w) * (fw / 2.0) + (fw / 2.0);  
                    let y2 = (v2.y / v2.w) * (fh / 2.0) + (fh / 2.0);  

                    let x3 = (v3.x / v3.w) * (fw / 2.0) + (fw / 2.0);  
                    let y3 = (v3.y / v3.w) * (fh / 2.0) + (fh / 2.0);  

                    super::draw_triangle_barycentric_z_uv(render_target,
                        &obj.texture,
                        &super::VertexUV{ x: x1, y: y1, z: v1.z / v1.w, w: 1.0, u: v1.u, v: v1.v },
                        &super::VertexUV{ x: x2, y: y2, z: v2.z / v2.w, w: 1.0, u: v2.u, v: v2.v },
                        &super::VertexUV{ x: x3, y: y3, z: v3.z / v3.w, w: 1.0, u: v3.u, v: v3.v },
                        );
                }
            }
        }
    }
}

impl Object {
    pub fn new() -> Object {
        let vs: Vec<math::Vector4> = Vec::new();
        let fs: Vec<Face> = Vec::new();
        let vns: Vec<math::Vector4> = Vec::new();
        let uvs: Vec<math::Point> = Vec::new();

        Object { vertices: vs, faces: fs, vertex_normals: vns, uvs: uvs, texture: super::Texture::new() }
    }
}

pub struct ObjLoader {

}

impl ObjLoader {
    fn parse_point(parts: &[&str]) -> math::Point {
        let x = parts[0].parse::<f32>();
        let y = parts[1].parse::<f32>();
        return math::Point {
            x: if x.is_ok() { x.unwrap() } else { 0.0 }, 
            y: if y.is_ok() { y.unwrap() } else { 0.0 },
        };
    }

    fn parse_vertex(parts: &[&str]) -> math::Vector4 {
        let x = parts[0].parse::<f32>();
        let y = parts[1].parse::<f32>();
        let z = parts[2].parse::<f32>();
        return math::Vector4 {
            x: if x.is_ok() { x.unwrap() } else { 0.0 }, 
            y: if y.is_ok() { y.unwrap() } else { 0.0 },
            z: if z.is_ok() { z.unwrap() } else { 0.0 },
            w: 1.0,
        };
    }

    fn parse_face_indexes(index_string: &str) -> (i32, i32, i32) {
        let indexes: Vec<&str> = index_string.split('/').collect();

        let mut fi: i32 = 0;
        let mut uvi: i32 = 0;
        let mut vni: i32 = 0;

        if indexes.len() > 0 {
            let res = indexes[0].parse::<i32>();
            fi = if res.is_ok() { res.unwrap() } else { 0 };
        }
        if indexes.len() > 1 {
            let res = indexes[1].parse::<i32>();
            uvi = if res.is_ok() { res.unwrap() } else { 0 };
        }
        if indexes.len() > 2 {
            let res = indexes[2].parse::<i32>();
            vni = if res.is_ok() { res.unwrap() } else { 0 };
        }

        return (
            fi,
            uvi,
            vni,
        );
    }

    pub fn load_obj(file_as_string: String) -> Object {
        let mut obj = self::Object::new();

        for line in file_as_string.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 0 {
                continue;
            }

            let entry_type = parts[0];
            let entry_data = &parts[1..];

            match entry_type.as_ref() {
                "v" => {
                    if entry_data.len() < 3 {
                        continue;
                    }
                    let vertex = self::ObjLoader::parse_vertex(entry_data);
                    obj.vertices.push(vertex);
                    // log!("vertex {:?}", vertex);
                },
                "vn" => {
                    if entry_data.len() < 3 {
                        continue;
                    }
                    let normal = self::ObjLoader::parse_vertex(entry_data);
                    obj.vertex_normals.push(normal);
                },
                "vt" => {
                    if entry_data.len() < 2 {
                        continue;
                    }
                    let uv = self::ObjLoader::parse_point(entry_data);
                    obj.uvs.push(uv);
                },
                "f" => {
                    // log!("{:?} {:?}", entry_data, entry_data.len());

                    if entry_data.len() != 3 && entry_data.len() != 4 {
                        continue;
                    }

                    let (i1, uvi1, vni1) = self::ObjLoader::parse_face_indexes(entry_data[0]);
                    let (i2, uvi2, vni2) = self::ObjLoader::parse_face_indexes(entry_data[1]);
                    let (i3, uvi3, vni3) = self::ObjLoader::parse_face_indexes(entry_data[2]);
                    
                    if entry_data.len() == 3 {
                        // triangle
                        let face = Face {
                            v0: i3 - 1,
                            v1: i2 - 1,
                            v2: i1 - 1,
                            vn0: vni3 - 1,
                            vn1: vni2 - 1,
                            vn2: vni1 - 1,
                            uv0: uvi3 - 1,
                            uv1: uvi2 - 1,
                            uv2: uvi1 - 1,
                        };
                        obj.faces.push(face);
                    } else if entry_data.len() == 4 {
                        let (i4, uvi4, vni4) = self::ObjLoader::parse_face_indexes(entry_data[3]);

                        let face = Face {
                            v0: i4 - 1,
                            v1: i2 - 1,
                            v2: i1 - 1,
                            vn0: vni4 - 1,
                            vn1: vni2 - 1,
                            vn2: vni1 - 1,
                            uv0: uvi4 - 1,
                            uv1: uvi2 - 1,
                            uv2: uvi1 - 1,
                        };
                        obj.faces.push(face);

                        let face2 = Face {
                            v0: i4 - 1,
                            v1: i3 - 1,
                            v2: i2 - 1,
                            vn0: vni4 - 1,
                            vn1: vni3 - 1,
                            vn2: vni2 - 1,
                            uv0: uvi4 - 1,
                            uv1: uvi3 - 1,
                            uv2: uvi2 - 1,
                        };
                        obj.faces.push(face2);
                    }
                },
                _ => {
                   
                }
            }
        }
        return obj;
    }
}

