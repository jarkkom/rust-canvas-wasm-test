
use std::mem;
use crate::math;

pub mod scene;

pub struct RenderTarget {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
    pub depth: Vec<f32>
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct VertexUV {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub u: f32,
    pub v: f32,
}

impl VertexUV {
    pub fn to_screen_point(&self) -> math::PointI32 {
        return math::PointI32 {
            x: (self.x + 0.5) as i32,
            y: (self.y + 0.5) as i32,
        }
    }

    pub fn lerp(&self, b: &VertexUV, a: f32) -> VertexUV {
        VertexUV{
            x: self.x * a + b.x * (1.0 - a),
            y: self.y * a + b.y * (1.0 - a),
            z: self.z * a + b.z * (1.0 - a),
            w: self.w * a + b.w * (1.0 - a),
            u: self.u * a + b.u * (1.0 - a),
            v: self.v * a + b.v * (1.0 - a),
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn new() -> Texture {
        Texture {
            width: 0,
            height: 0,
            data: vec![],
        }
    }
}

// render top half of triangle (y2 == y3)
fn draw_triangle_top(target: &mut RenderTarget, c: &Color, p1: math::Point, p2: math::Point, p3: math::Point) {
    let dx1 = (p3.x - p1.x) / (p3.y - p1.y);
    let dx2 = (p2.x - p1.x) / (p2.y - p1.y);

    let mut cx1 = p1.x;
    let mut cx2 = p1.x;

    // log!("top {} -> {}, dx {} {}", y1, y2, dx1, dx2);

    let ys = (p1.y + 0.5) as u32;
    let ye = (p2.y + 0.5) as u32;
    for y in ys..ye {
        let mut tsx = cx1;
        let mut tex = cx2;
        if tex < tsx {
            mem::swap(&mut tsx, &mut tex);
        }
        let sx = (tsx + 0.5) as u32;
        let ex = (tex + 0.5) as u32;

        let mut i = (((y * target.width) + sx) * 4u32) as usize;
        for _x in sx..ex {
            target.buffer[i + 0] = c.r;
            target.buffer[i + 1] = c.g;
            target.buffer[i + 2] = c.b;
            target.buffer[i + 3] = c.a;
            i += 4
        }
        cx1 += dx1;
        cx2 += dx2;
    }
}

// render bottom half of triangle (y1 == y2)
fn draw_triangle_bottom(target: &mut RenderTarget, c: &Color, p1: math::Point, p2: math::Point, p3: math::Point) {
    let dx1 = (p3.x - p1.x) / (p3.y - p1.y);
    let dx2 = (p3.x - p2.x) / (p3.y - p2.y);

    let mut cx1 = p1.x;
    let mut cx2 = p2.x;

    let ys = (p2.y + 0.5) as u32;
    let ye = (p3.y + 0.5) as u32;
    for y in ys..ye {
        let mut tsx = cx1;
        let mut tex = cx2;
        if tex < tsx {
            mem::swap(&mut tsx, &mut tex);
        }
        let sx = (tsx + 0.5) as u32;
        let ex = (tex + 0.5) as u32;

        let mut i = (((y * target.width) + sx) * 4u32) as usize;
        for _x in sx..ex {
            target.buffer[i + 0] = c.r;
            target.buffer[i + 1] = c.g;
            target.buffer[i + 2] = c.b;
            target.buffer[i + 3] = c.a;
            i += 4
        }
        cx1 += dx1;
        cx2 += dx2;
    }
}

pub fn draw_triangle(target: &mut RenderTarget, c: &Color, p1: math::Point, p2: math::Point, p3: math::Point) {
    let mut high = p1;
    let mut mid = p2;
    let mut low = p3;

    // log!("pre {},{} {},{} {},{}", x1, y1, x2, y2, x3, y3);

    // sort vertices
    if high.y > mid.y {
        mem::swap(&mut high, &mut mid);
    }
    if high.y > low.y {
        mem::swap(&mut high, &mut low);
    }
    if mid.y > low.y {
        mem::swap(&mut mid, &mut low);
    }

    // log!("srt {},{} {},{} {},{}", hi_x, hi_y, mi_x, mi_y, lo_x, lo_y);

    if mid.y == low.y {
        draw_triangle_top(target, c, high, mid, low);
    } else if high.y == mid.y {
        draw_triangle_bottom(target, c, high, mid, low);
    } else {
        let p4 = math::Point{
            x: high.x + ((mid.y - high.y) / (low.y - high.y)) * (low.x - high.x),
            y: mid.y,
        };

        draw_triangle_top(target, c, high, mid, p4);
        draw_triangle_bottom(target, c, mid, p4, low);
    }
}

fn orient2d(a: &math::PointI32, b: &math::PointI32, cx: i32, cy: i32) -> i32 {
    return (b.x-a.x)*(cy-a.y) - (b.y-a.y)*(cx-a.x);
}

fn orient2df(a: &math::Point, b: &math::Point, cx: f32, cy: f32) -> f32 {
    return (b.x-a.x)*(cy-a.y) - (b.y-a.y)*(cx-a.x);
}

fn to_screen_point(p: &math::Point) -> math::PointI32 {
    return math::PointI32 {
        x: (p.x + 0.5) as i32,
        y: (p.y + 0.5) as i32,
    }
}

fn vec3_to_screen_point(p: &math::Vector3) -> math::PointI32 {
    return math::PointI32 {
        x: (p.x + 0.5) as i32,
        y: (p.y + 0.5) as i32,
    }
}

pub fn draw_triangle_barycentric(target: &mut RenderTarget, c: &Color, p1: math::Point, p2: math::Point, p3: math::Point) {
    let v0 = to_screen_point(&p1);
    let v1 = to_screen_point(&p2);
    let v2 = to_screen_point(&p3);

    let minx = (v0.x.min(v1.x.min(v2.x)) as i32).max(0);
    let miny = (v0.y.min(v1.y.min(v2.y)) as i32).max(0);
    let maxx = (v0.x.max(v1.x.max(v2.x)) as i32).min(target.width as i32 - 1);
    let maxy = (v0.y.max(v1.y.max(v2.y)) as i32).min(target.height as i32 - 1);

    let a01 = v0.y - v1.y;
    let b01 = v1.x - v0.x;
    let a12 = v1.y - v2.y;
    let b12 = v2.x - v1.x;
    let a20 = v2.y - v0.y;
    let b20 = v0.x - v2.x;

    let mut w0_row = orient2d(&v1, &v2, minx, miny);
    let mut w1_row = orient2d(&v2, &v0, minx, miny);
    let mut w2_row = orient2d(&v0, &v1, minx, miny);

    for _y in miny..maxy {
        let mut w0 = w0_row;
        let mut w1 = w1_row;
        let mut w2 = w2_row;

        let start_i = (((_y as u32 * target.width) + minx as u32) * 4u32) as usize;
        let end_i = start_i + target.width as usize;

        let slice = &mut target.buffer[start_i..end_i];
        for x in slice.chunks_exact_mut(4) {
            if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                x[0] = c.r;
                x[1] = c.g;
                x[2] = c.b;
                x[3] = c.a;
            }
            w0 += a12;
            w1 += a20;
            w2 += a01;
        }

        // for _x in minx..maxx {
        //     if w0 >= 0 && w1 >= 0 && w2 >= 0 {
        //         target.buffer[i + 0] = c.r;
        //         target.buffer[i + 1] = c.g;
        //         target.buffer[i + 2] = c.b;
        //         target.buffer[i + 3] = c.a;
        //     }
        //     w0 += a12;
        //     w1 += a20;
        //     w2 += a01;
        //     i += 4;
        // }
        w0_row += b12;
        w1_row += b20;
        w2_row += b01;
    }
}

pub fn draw_triangle_barycentric_z(target: &mut RenderTarget, c: &Color, p0: math::Vector3, p1: math::Vector3, p2: math::Vector3) {
    let v0 = vec3_to_screen_point(&p2);
    let v1 = vec3_to_screen_point(&p1);
    let v2 = vec3_to_screen_point(&p0);

    let z0 = p2.z;
    let z1 = p1.z;
    let z2 = p0.z;

    let minx = (v0.x.min(v1.x.min(v2.x)) as i32).max(0);
    let miny = (v0.y.min(v1.y.min(v2.y)) as i32).max(0);
    let maxx = (v0.x.max(v1.x.max(v2.x)) as i32).min((target.width as i32) - 1);
    let maxy = (v0.y.max(v1.y.max(v2.y)) as i32).min((target.height as i32) - 1);

    if maxx < 0 || maxy < 0 || miny > target.height as i32|| minx > target.width as i32 {
        return;
    }

    let a0 = v1.y - v2.y;
    let a1 = v2.y - v0.y;
    let a2 = v0.y - v1.y;

    let b0 = v2.x - v1.x;
    let b1 = v0.x - v2.x;
    let b2 = v1.x - v0.x;

    let c0 = v1.x * v2.y - v2.x * v1.y;
    let c1 = v2.x * v0.y - v0.x * v2.y;
    let c2 = v0.x * v1.y - v1.x * v0.y;

    let area = (v1.x - v0.x) * (v2.y - v0.y) - (v0.x - v2.x) * (v0.y - v1.y);
    let r_area = 1.0 / area as f32;

    let mut w0_row = (a0 * minx) + (b0 * miny) + c0;
    let mut w1_row = (a1 * minx) + (b1 * miny) + c1;
    let mut w2_row = (a2 * minx) + (b2 * miny) + c2;

    let zz1 = (z1 - z0) * r_area;
    let zz2 = (z2 - z0) * r_area;

    for _y in miny..maxy {
        let mut w0 = w0_row;
        let mut w1 = w1_row;
        let mut w2 = w2_row;

        let start_i = (((_y as u32 * target.width) + minx as u32) * 4u32) as usize;
        let end_i = (((_y as u32 * target.width) + (maxx + 1) as u32) * 4u32) as usize;

        let slice = &mut target.buffer[start_i..end_i];
        let depth_slice = &mut target.depth[start_i / 4usize..end_i / 4usize];

        let mut depth_iter = depth_slice.iter_mut();

        for x in slice.chunks_exact_mut(4) {
            let depth_buffer = depth_iter.next().unwrap();
            if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                let z = z0 + zz1 * w1 as f32 + zz2 * w2 as f32;
                if *depth_buffer < z {
                    x[0] = c.r;
                    x[1] = c.g;
                    x[2] = c.b;
                    x[3] = c.a;
                    *depth_buffer = z;
                }
        }
            w0 += a0;
            w1 += a1;
            w2 += a2;
        }

        w0_row += b0;
        w1_row += b1;
        w2_row += b2;
    }
}

pub fn draw_triangle_barycentric_z_uv(target: &mut RenderTarget, texture: &Texture, p0: &VertexUV, p1: &VertexUV, p2: &VertexUV) {
    let v0 = p2.to_screen_point();
    let v1 = p1.to_screen_point();
    let v2 = p0.to_screen_point();

    let z0 = p2.z;
    let z1 = p1.z;
    let z2 = p0.z;

    let tu0 = p2.u;
    let tv0 = p2.v;
    let tu1 = p1.u;
    let tv1 = p1.v;
    let tu2 = p0.u;
    let tv2 = p0.v;

    let minx = (v0.x.min(v1.x.min(v2.x)) as i32).max(0);
    let miny = (v0.y.min(v1.y.min(v2.y)) as i32).max(0);
    let maxx = (v0.x.max(v1.x.max(v2.x)) as i32).min((target.width as i32) - 1);
    let maxy = (v0.y.max(v1.y.max(v2.y)) as i32).min((target.height as i32) - 1);

    if maxx < 0 || maxy < 0 || miny > target.height as i32|| minx > target.width as i32 {
        return;
    }

    let a0 = v1.y - v2.y;
    let a1 = v2.y - v0.y;
    let a2 = v0.y - v1.y;

    let b0 = v2.x - v1.x;
    let b1 = v0.x - v2.x;
    let b2 = v1.x - v0.x;

    let c0 = v1.x * v2.y - v2.x * v1.y;
    let c1 = v2.x * v0.y - v0.x * v2.y;
    let c2 = v0.x * v1.y - v1.x * v0.y;

    let area = (v1.x - v0.x) * (v2.y - v0.y) - (v0.x - v2.x) * (v0.y - v1.y);
    let r_area = 1.0 / area as f32;

    let mut w0_row = (a0 * minx) + (b0 * miny) + c0;
    let mut w1_row = (a1 * minx) + (b1 * miny) + c1;
    let mut w2_row = (a2 * minx) + (b2 * miny) + c2;

    let zz1 = (z1 - z0) * r_area;
    let zz2 = (z2 - z0) * r_area;

    let ttu1 = (tu1 - tu0) * r_area;
    let ttv1 = (tv1 - tv0) * r_area;
    let ttu2 = (tu2 - tu0) * r_area;
    let ttv2 = (tv2 - tv0) * r_area;

    for _y in miny..maxy {
        let mut w0 = w0_row;
        let mut w1 = w1_row;
        let mut w2 = w2_row;

        let start_i = (((_y as u32 * target.width) + minx as u32) * 4u32) as usize;
        let end_i = (((_y as u32 * target.width) + (maxx + 1) as u32) * 4u32) as usize;

        let slice = &mut target.buffer[start_i..end_i];
        let depth_slice = &mut target.depth[start_i / 4usize..end_i / 4usize];

        let mut depth_iter = depth_slice.iter_mut();

        for x in slice.chunks_exact_mut(4) {
            let depth_buffer = depth_iter.next().unwrap();
            if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                let z = z0 + zz1 * w1 as f32 + zz2 * w2 as f32;

                let u = ((tu0 + ttu1 * w1 as f32 + ttu2 * w2 as f32) * texture.width as f32) as u32 & (texture.width - 1);
                let v = ((tv0 + ttv1 * w1 as f32 + ttv2 * w2 as f32) * texture.height as f32) as u32 & (texture.height - 1);

                if *depth_buffer > z {
                    x[0] = texture.data[((v * texture.width) + u) as usize * 4 + 0];
                    x[1] = texture.data[((v * texture.width) + u) as usize * 4 + 1];
                    x[2] = texture.data[((v * texture.width) + u) as usize * 4 + 2];
                    x[3] = 255;
                    *depth_buffer = z;
                }
            }
            w0 += a0;
            w1 += a1;
            w2 += a2;
        }

        w0_row += b0;
        w1_row += b1;
        w2_row += b2;
    }
}
