use glam::Vec2;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};

use crate::utils::{vec2_to_point2,distance};
use crate::human::Human;


const ZOMBIE_SPEED: f32 = 1.0;

pub struct Zombie {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Zombie {
    pub fn update(&mut self, ctx: &mut Context, humans: &[Human]) {
        if let Some(target) = self.closest_human(humans) {
            #[cfg(debug_assertions)]
            {
                println!("Following human");
            }
            let dir = (target.position - self.position).normalize();
            self.velocity = dir * ZOMBIE_SPEED; 
        }
    
        self.position += self.velocity;
    
        if self.position.x < 0.0 || self.position.x > 800.0 {
            self.velocity.x = -self.velocity.x;
        }

        if self.position.y < 0.0 || self.position.y > 600.0 {
            self.velocity.y = -self.velocity.y;
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2_to_point2(self.position),
            5.0,  
            0.1,  
            Color::RED,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())
    }

    pub fn closest_human<'a>(&self, humans: &'a [Human]) -> Option<&'a Human> {
        humans.iter().min_by(move |a, b| {
            let dist_a = distance(self.position, a.position);
            let dist_b = distance(self.position, b.position);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
    }
    
}
