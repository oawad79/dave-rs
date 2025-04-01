use macroquad::math::{
    Rect,
    Vec2,
};

pub trait Collidable {
    /// Returns the collision rectangle for this entity
    fn get_collision_rect(&self) -> Rect;

    /// Returns the current position of this entity
    fn get_position(&self) -> Vec2;

    /// Handle being hit by something
    fn on_hit(&mut self);

    /// Check if this entity is alive
    fn is_alive(&self) -> bool;
}
