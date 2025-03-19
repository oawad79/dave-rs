
#[derive(Debug, Clone, PartialEq)]
pub enum BulletDirection {
    Left,
    Right
}
#[derive(Debug, Clone)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub collided: bool,
    pub direction: BulletDirection
}

