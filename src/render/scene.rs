use crate::math;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
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
}

impl Scene {
    pub fn new() -> Scene {
        let objs: Vec<Object> = vec![];

        Scene { objects: objs }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }
}

impl Object {
    pub fn new() -> Object {
        let vs: Vec<math::Vector4> = Vec::new();
        let fs: Vec<Face> = Vec::new();
        let vns: Vec<math::Vector4> = Vec::new();
        let uvs: Vec<math::Point> = Vec::new();

        Object { vertices: vs, faces: fs, vertex_normals: vns, uvs: uvs }
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
                            v0: i1 - 1,
                            v1: i2 - 1,
                            v2: i3 - 1,
                            vn0: vni1 - 1,
                            vn1: vni2 - 1,
                            vn2: vni3 - 1,
                            uv0: uvi1 - 1,
                            uv1: uvi2 - 1,
                            uv2: uvi3 - 1,
                        };
                        obj.faces.push(face);
                    } else if entry_data.len() == 4 {
                        let (i4, uvi4, vni4) = self::ObjLoader::parse_face_indexes(entry_data[3]);

                        let face = Face {
                            v0: i1 - 1,
                            v1: i2 - 1,
                            v2: i4 - 1,
                            vn0: vni1 - 1,
                            vn1: vni2 - 1,
                            vn2: vni4 - 1,
                            uv0: uvi1 - 1,
                            uv1: uvi2 - 1,
                            uv2: uvi4 - 1,
                        };
                        obj.faces.push(face);

                        let face2 = Face {
                            v0: i2 - 1,
                            v1: i3 - 1,
                            v2: i4 - 1,
                            vn0: vni2 - 1,
                            vn1: vni3 - 1,
                            vn2: vni4 - 1,
                            uv0: uvi2 - 1,
                            uv1: uvi3 - 1,
                            uv2: uvi4 - 1,
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

