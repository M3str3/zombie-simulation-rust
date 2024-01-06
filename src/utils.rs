use glam::Vec2;
use crate::collisions::Collidable;

pub fn vec2_to_point2(v: Vec2) -> ggez::mint::Point2<f32> {
    ggez::mint::Point2 { x: v.x, y: v.y }
}

pub fn distance(a: Vec2, b: Vec2) -> f32 {
    a.distance(b)
}

pub fn is_near<A: Collidable, B: Collidable>(entity1: &A, entity2: &B, threshold: f32) -> bool {
    distance(entity1.position(), entity2.position()) < threshold
}