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
        self.min.x <= point_xyz.x && point_xyz.x <= self.max.x &&
        self.min.y <= point_xyz.y && point_xyz.y <= self.max.y &&
        self.min.z <= point_xyz.z && point_xyz.z <= self.max.z
    }

    pub fn get_center(&self) -> Vec3d {
        Vec3d::new((self.min.x + self.max.x) / 2.0, (self.min.y + self.max.y) / 2.0, (self.min.z + self.max.z) / 2.0)
    }

    pub fn shift(&mut self, by: Vec3d) {
        self.min += by;
        self.max += by;
    }

    pub fn overlaps(&self, other: BoundingBox) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.x
    }

    pub fn resize(&mut self, min_xyz: Vec3d, max_xyz: Vec3d) {
        self.min = min_xyz;
        self.max = max_xyz;
    }
}