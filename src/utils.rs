use glam::Vec2;

pub fn vec2_to_point2(v: Vec2) -> ggez::mint::Point2<f32> {
    ggez::mint::Point2 { x: v.x, y: v.y }
}

pub fn distance(a: Vec2, b: Vec2) -> f32 {
    a.distance(b)
}
