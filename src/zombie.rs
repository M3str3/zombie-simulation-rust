use glam::Vec2;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};

use crate::config::Config;
use crate::utils::{vec2_to_point2,distance};
use crate::human::Human;

const ZOMBIE_VISION_RANGE: f32 = 150.0; // o cualquier otro valor que consideres adecuado

pub struct Zombie {
    pub position: Vec2,
    pub speed: Vec2,
    pub speed_chasing: Vec2,
}

impl Zombie {
    pub fn update(&mut self, _ctx: &mut Context, humans: &[Human], config:&Config ) {
        let speed;
        if let Some(target) = self.closest_human(humans, config.zombie_vision_range) {
            #[cfg(debug_assertions)]
            println!("Following human");
            let dir = (target.position - self.position).normalize();
            speed = dir * self.speed_chasing; 
        }else{
            speed = self.speed;
        }
    
        self.position += speed;
    
        if self.position.x < 0.0 || self.position.x > 800.0 {
            self.speed.x = -self.speed.x;
        }

        if self.position.y < 0.0 || self.position.y > 600.0 {
            self.speed.y = -self.speed.y;
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


    pub fn closest_human<'a>(&self, humans: &'a [Human], vision_range: f32) -> Option<&'a Human> {
        humans.iter()
            .filter(|&human| distance(self.position, human.position) < vision_range)
            .min_by(|a, b| {
                let dist_a = distance(self.position, a.position);
                let dist_b = distance(self.position, b.position);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
    }
    
    
}
