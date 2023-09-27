use glam::Vec2;
use crate::zombie::Zombie;
use crate::human::Human;

pub fn vec2_to_point2(v: Vec2) -> ggez::mint::Point2<f32> {
    ggez::mint::Point2 { x: v.x, y: v.y }
}

pub fn distance(a: Vec2, b: Vec2) -> f32 {
    a.distance(b)
}

pub fn is_near(human: &Human, zombie: &Zombie, threshold: f32) -> bool {
    distance(human.position, zombie.position) < threshold
}