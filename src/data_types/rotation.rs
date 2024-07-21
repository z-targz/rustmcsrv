use super::Angle;

#[derive(Debug, Copy, Clone)]
pub struct Rotation {
    pub yaw: Angle, 
    pub pitch: Angle,
}