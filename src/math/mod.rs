
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct PointI32 {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl Point {
    pub fn rotate(&self, angle: f32) -> Point {
      Point {
        x: angle.cos() * self.x - angle.sin() * self.y,
        y: angle.cos() * self.y + angle.sin() * self.x,        
      } 
    }
    pub fn translate(&self, x: f32, y: f32) -> Point {
      Point {
        x: self.x + x,
        y: self.y + y,        
      } 
    }

    pub fn eq(&self, other: &Self) -> bool {
      self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

#[derive(Debug)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

#[allow(dead_code)]
impl Vector3 {
  pub fn new() -> Vector3 {
    Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }

  pub fn normal(&self) -> Vector3 {
    let len = self.len();
    if len < 1e-8 {
      if self.x >= self.y && self.x >= self.z {
        return Vector3{ x: 1.0, y: 0.0, z: 0.0 };
      }
      if self.y >= self.z {
        return Vector3{ x: 0.0, y: 1.0, z: 0.0 };
      }
      return Vector3{ x: 0.0, y: 0.0, z: 1.0 };
    }
    Vector3 {
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
    }
  }

  pub fn len(&self) -> f32 {
    return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
  }

  pub fn negate(&self) -> Vector3 {
    Vector3 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }

  pub fn cross(&self, v: &Vector3) -> Vector3 {
    Vector3 {
      x: self.y * v.z - self.z * v.y,
      y: self.x * v.z - self.z * v.x,
      z: self.x * v.y - self.y * v.x,
    }
  }

  pub fn sub(&self, v: &Vector3) -> Vector3 {
    Vector3 {
      x: self.x - v.x,
      y: self.y - v.y,
      z: self.z - v.z,
    }
  }

  pub fn dot(&self, v: &Vector3) -> f32 {
    return self.x*v.x + self.y*v.y + self.z*v.z;
  }
}

#[derive(Debug, PartialEq)]
pub struct Vector4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

#[allow(dead_code)]
impl Vector4 {
  pub fn new() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0,
      w: 1.0,
    }
  }

  pub fn multiply(&self, m: &Matrix4) -> Self {
    Self {
      x: self.x * m.m[0][0] + self.y * m.m[1][0] + self.z * m.m[2][0] + self.w * m.m[3][0],
      y: self.x * m.m[0][1] + self.y * m.m[1][1] + self.z * m.m[2][1] + self.w * m.m[3][1],
      z: self.x * m.m[0][2] + self.y * m.m[1][2] + self.z * m.m[2][2] + self.w * m.m[3][2],
      w: self.x * m.m[0][3] + self.y * m.m[1][3] + self.z * m.m[2][3] + self.w * m.m[3][3],
    }
  }

  pub fn normal(&self) -> Self {
    let len = (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt();
    Self {
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
      w: self.w / len,
    }
  }

  pub fn add(&self, v: &Self) -> Self {
    Self {
      x: self.x + v.x,
      y: self.y + v.y,
      z: self.z + v.z,
      w: self.w + v.w,
    }
  }

  pub fn sub(&self, v: &Self) -> Self {
    Self {
      x: self.x - v.x,
      y: self.y - v.y,
      z: self.z - v.z,
      w: self.w - v.w,
    }
  }

  pub fn scale(&self, a: f32) -> Self {
    Self {
      x: self.x * a,
      y: self.y * a,
      z: self.z * a,
      w: self.w * a,
    }
  }

  pub fn dot(&self, v: &Self) -> f32 {
    return self.x*v.x + self.y*v.y + self.z*v.z + self.w*v.w;
  }

  pub fn cross(&self, v: &Self) -> Self {
    Self {
      x: self.y * v.z - self.z * v.y,
      y: self.x * v.z - self.z * v.x,
      z: self.x * v.y - self.y * v.x,
      w: 1.0,
    }
  }

  pub fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
  }
}

#[derive(Debug)]
pub struct Matrix4 {
  pub m: [[f32; 4]; 4],
}

#[allow(dead_code)]
impl Matrix4 {
  pub fn zero() -> Matrix4 {
    Matrix4 {
      m: [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        ]
    }
  }

  pub fn identity() -> Matrix4 {
    Matrix4 {
      m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
        ]
    }
  }

  pub fn rotation_x(angle: f32) -> Matrix4 {
    Matrix4 {
      m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, angle.cos(), -angle.sin(), 0.0],
        [0.0, angle.sin(), angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ]
    }
  }

  pub fn rotation_y(angle: f32) -> Matrix4 {
    Matrix4 {
      m: [
        [angle.cos(), 0.0, angle.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-angle.sin(), 0.0, -angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ]
    }
  }

  pub fn rotation_z(angle: f32) -> Matrix4 {
    Matrix4 {
      m: [
        [angle.cos(), -angle.sin(), 0.0, 0.0],
        [angle.sin(), angle.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ]
    }
  }

  pub fn projection(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
    let f = 1.0 / (fov / 2.0).tan();
    let inv_r = 1.0 / (near - far);

    Matrix4 {
      m: [
        [f / aspect, 0.0, 0.0, 0.0],
        [0.0, -f, 0.0, 0.0],
        [0.0, 0.0, (far) / (far-near), 1.0],
        [0.0, 0.0, near * far * inv_r, 0.0],
        ]
    }
  }

  pub fn lookat_rot(eye: &Vector3, target: &Vector3) -> Matrix4 {
    let mut mat = Matrix4::identity();
    let forward = target.sub(eye);
    let focus = forward.len();

    let ax = -(forward.x.atan2(forward.z));
    let ay = (forward.y / focus).asin();
    let az = 0.0 as f32;

    let sinx = ax.sin();
    let cosx = ax.cos();
    let siny = ay.sin();
    let cosy = ay.cos();
    let sinz = az.sin();
    let cosz = az.cos();

    mat.m[0][0] =  sinx * siny * sinz + cosx * cosz;
    mat.m[1][0] =  cosy * sinz;
    mat.m[2][0] =  sinx * cosz - cosx * siny * sinz;
    mat.m[0][1] =  sinx * siny * cosz - cosx * sinz;
    mat.m[1][1] =  cosy * cosz;
    mat.m[2][1] = -cosx * siny * cosz - sinx * sinz;
    mat.m[0][2] = -sinx * cosy;
    mat.m[1][2] =  siny;
    mat.m[2][2] =  cosx * cosy;

    return mat;
  }

  pub fn lookat(eye: &Vector3, target: &Vector3) -> Matrix4 {
    let mut mat = Matrix4::lookat_rot(eye, target);
    let pivot = eye.negate();

    mat.m[3][0] = mat.m[0][0] * pivot.x + mat.m[1][0] * pivot.y + mat.m[2][0] * pivot.z + mat.m[3][0];
    mat.m[3][1] = mat.m[0][1] * pivot.x + mat.m[1][1] * pivot.y + mat.m[2][1] * pivot.z + mat.m[3][1];
    mat.m[3][2] = mat.m[0][2] * pivot.x + mat.m[1][2] * pivot.y + mat.m[2][2] * pivot.z + mat.m[3][2];

    return mat;
  }

  pub fn translate_xyz(&self, x: f32, y: f32, z: f32) -> Matrix4 {
    let mut res = Matrix4 {
      m: [ 
        [self.m[0][0], self.m[0][1], self.m[0][2], self.m[0][3]],
        [self.m[1][0], self.m[1][1], self.m[1][2], self.m[1][3]],
        [self.m[2][0], self.m[2][1], self.m[2][2], self.m[2][3]],
        [0.0, 0.0, 0.0, self.m[3][3]],
      ]
    };
    for i in 0..4 {
      res.m[3][i] += x * self.m[0][i] + y * self.m[1][i] + z * self.m[2][i];
    }

    return res;
  }

  pub fn translate(&self, translate: &Vector3) -> Matrix4 {
    Matrix4 {
      m: [ 
        [self.m[0][0], self.m[0][1], self.m[0][2], self.m[0][3]],
        [self.m[1][0], self.m[1][1], self.m[1][2], self.m[1][3]],
        [self.m[2][0], self.m[2][1], self.m[2][2], self.m[2][3]],
        [translate.x, translate.y, translate.z, self.m[3][3]],
      ]
    }
  }

  pub fn multiply(&self, b: &Matrix4) -> Matrix4 {
    let a = self;
    let mut res = Self::zero();
    for col in 0..4 {
      for row in 0..4 {
        let mut s = 0.0;
        for k in 0..4 {
          s += a.m[row][k] * b.m[k][col];
        }
        res.m[row][col] = s;
      }
    }
    return res;
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector4_add() {
        let a = Vector4{ x: 10.0, y: 20.0, z: 30.0, w: 40.0 };
        let expected = Vector4{ x: 11.1, y: 22.2, z: 33.3, w: 44.4 };
        assert_eq!(expected, a.add(&Vector4{ x: 1.1, y: 2.2, z: 3.3, w: 4.4 }));
    }

    #[test]
    fn test_vector4_sub() {
        let a = Vector4{ x: 11.1, y: 22.2, z: 33.3, w: 44.4 };
        let expected = Vector4{ x: 10.0, y: 20.0, z: 30.0, w: 40.0 };
        assert_eq!(expected, a.sub(&Vector4{ x: 1.1, y: 2.2, z: 3.3, w: 4.4 }));
    }

    #[test]
    fn test_vector4_scale() {
        let a = Vector4{ x: 1.1, y: 2.2, z: 3.3, w: 4.4 };
        let expected = Vector4{ x: 11.0, y: 22.0, z: 33.0, w: 44.0 };
        assert_eq!(expected, a.scale(10.0));
    }

    #[test]
    fn test_vector4_dot() {
        let a = Vector4{ x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
        assert_eq!(40.0 + 90.0 + 160.0 + 250.0, a.dot(&Vector4{ x: 20.0, y: 30.0, z: 40.0, w: 50.0 }));
    }


    #[test]
    fn test_vector_matrix4_multiply() {
        let vertex = Vector4 { x: 4.0, y: 3.0, z: 2.0, w: 1.0 };
        let matrix = Matrix4::identity();
        let transformed = vertex.multiply(&matrix);

        assert_eq!(transformed, vertex);       
    }
}