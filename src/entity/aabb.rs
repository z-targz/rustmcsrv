use crate::data_types::Vec3d;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    max: Vec3d,
    min: Vec3d,
}

impl BoundingBox {
    pub fn new(min_xyz: Vec3d, max_xyz: Vec3d) -> Self {
        BoundingBox {
            min : min_xyz,
            max : max_xyz,
        }
    }

    pub fn contains(&self, point_xyz: Vec3d) -> bool {
        self.min.0 <= point_xyz.0 && point_xyz.0 <= self.max.0 &&
        self.min.1 <= point_xyz.1 && point_xyz.1 <= self.max.1 &&
        self.min.2 <= point_xyz.2 && point_xyz.2 <= self.max.2
    }

    pub fn get_center(&self) -> Vec3d {
        Vec3d((self.min.0 + self.max.0) / 2.0, (self.min.1 + self.max.1) / 2.0, (self.min.2 + self.max.2) / 2.0)
    }

    pub fn shift(&mut self, by: Vec3d) {
        self.min += by;
        self.max += by;
    }

    pub fn overlaps(&self, other: BoundingBox) -> bool {
        self.min.0 <= other.max.0 && self.max.0 >= other.min.0 &&
        self.min.1 <= other.max.1 && self.max.1 >= other.min.1 &&
        self.min.2 <= other.max.2 && self.max.2 >= other.min.0
    }

    pub fn resize(&mut self, min_xyz: Vec3d, max_xyz: Vec3d) {
        self.min = min_xyz;
        self.max = max_xyz;
    }
}