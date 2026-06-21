#[derive(Clone, Copy, Default)]
pub struct Vector3D {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl std::ops::Index<usize> for Vector3D {
  type Output = f32;

  fn index(&self, i: usize) -> &Self::Output {
    if let 0 = i {
      &self.x
    } else if let 1 = i {
      &self.y
    } else {
      &self.z
    }
  }
}
impl std::ops::IndexMut<usize> for Vector3D {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    if let 0 = i {
      &mut self.x
    } else if let 1 = i {
      &mut self.y
    } else {
      &mut self.z
    }
  }
}

impl Eq for Vector3D {}

impl PartialEq for Vector3D {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
  // Provided method
  fn ne(&self, other: &Self) -> bool {
    self.x != other.x || self.y != other.y || self.z != other.z
  }
}

impl Ord for Vector3D {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    // TODO: this is temp, decide how this should be compared
    // (self.x, self.y, self.z).total_cmp(other.x, other.y, other.z)
    // self.x.cmp(other.x) && self.y.cmp(other.y)
    self.x.total_cmp(&other.x)
  }

  // Provided methods
  fn max(self, other: Self) -> Self {
    Vector3D {
      x: self.x.max(other.x),
      y: self.y.max(other.y),
      z: self.z.max(other.z),
    }
  }
  fn min(self, other: Self) -> Self {
    Vector3D {
      x: self.x.min(other.x),
      y: self.y.min(other.y),
      z: self.z.min(other.z),
    }
  }
  fn clamp(self, min: Self, max: Self) -> Self {
    Vector3D {
      x: self.x.clamp(min.x, max.x),
      y: self.y.clamp(min.y, max.y),
      z: self.z.clamp(min.z, max.z),
    }
  }
}

impl PartialOrd for Vector3D {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.x.partial_cmp(&other.x)
  }

  fn lt(&self, other: &Self) -> bool {
    self.x < other.x && self.y < other.y && self.z < other.z
  }
  fn le(&self, other: &Self) -> bool {
    self.x <= other.x && self.y <= other.y && self.z <= other.z
  }
  fn gt(&self, other: &Self) -> bool {
    self.x > other.x && self.y > other.y && self.z > other.z
  }
  fn ge(&self, other: &Self) -> bool {
    self.x >= other.x && self.y >= other.y && self.z >= other.z
  }
}

impl Vector3D {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }
  pub fn zero() -> Self {
    Self {
      x: 0.,
      y: 0.,
      z: 0.,
    }
  }

  // NOTE: Not really needed
  pub fn x(&self) -> f32 {
    self[0]
  }
  pub fn y(&self) -> f32 {
    self[1]
  }
  pub fn z(&self) -> f32 {
    self[2]
  }

  pub fn dist2(&self, other: Vector3D) -> f32 {
    let diff = *self - other;
    diff.x.powf(2.0) + diff.y.powf(2.0) + diff.z.powf(2.0)
  }

  pub fn dist(&self, other: Vector3D) -> f32 {
    self.dist2(other).sqrt()
  }

  // TODO: remove or move to debug
  pub fn print(vector: Vector3D) -> String {
    format!("<{}, {}, {}> ##############", vector.x, vector.y, vector.z)
  }
}

// TODO: move to debug, or export/import lib
impl std::fmt::Display for Vector3D {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
  }
}

// TODO: possibly move to operations lib for organization purposes
impl std::ops::Add for Vector3D {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Vector3D {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl std::ops::Sub for Vector3D {
  fn sub(self, rhs: Self) -> Self::Output {
    Vector3D {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }

  type Output = Self;
}
impl std::ops::Add<f32> for Vector3D {
  type Output = Self;

  fn add(self, rhs: f32) -> Self::Output {
    Vector3D {
      x: self.x + rhs,
      y: self.y + rhs,
      z: self.z + rhs,
    }
  }
}

impl std::ops::Sub<f32> for Vector3D {
  fn sub(self, rhs: f32) -> Self::Output {
    Vector3D {
      x: self.x - rhs,
      y: self.y - rhs,
      z: self.z - rhs,
    }
  }

  type Output = Self;
}

impl std::ops::Mul<f32> for Vector3D {
  fn mul(self, rhs: f32) -> Self::Output {
    Vector3D {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }

  type Output = Self;
}

impl std::ops::Div<f32> for Vector3D {
  fn div(self, rhs: f32) -> Self::Output {
    Vector3D {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }

  type Output = Self;
}
