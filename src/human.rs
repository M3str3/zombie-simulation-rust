use glam::Vec2;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};

use crate::utils::vec2_to_point2;

pub struct Human {
    pub position: Vec2,
    pub velocity: Vec2,
    pub time_near_zombie: f32,
    pub is_infected: bool,
}

impl Human {

    pub fn update(&mut self, _ctx: &mut Context) {
        self.position += self.velocity;

        if self.position.x < 0.0 || self.position.x > 800.0 {
            self.velocity.x = -self.velocity.x;
        }

        if self.position.y < 0.0 || self.position.y > 600.0 {
            self.velocity.y = -self.velocity.y;
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let color = if self.is_infected {
            Color::YELLOW // Cambia esto al color que quieras para representar un humano infectado.
        } else {
            Color::BLUE
        };

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2_to_point2(self.position),
            5.0,  
            0.1,  
            color,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())
    }
}
