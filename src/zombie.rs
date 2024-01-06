use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::Vec2;

use crate::config::Config;
use crate::human::Human;
use crate::utils::{distance, vec2_to_point2};

#[derive(Clone, PartialEq)]
pub struct Zombie {
    pub position: Vec2,
    pub speed_current: Vec2,
    pub speed: Vec2,
    pub speed_chasing: Vec2,
}

impl Zombie {
    pub fn update(&mut self, ctx: &mut Context, humans: &mut [Human], config: &Config) {
        let (win_width, win_height) = graphics::drawable_size(ctx);
        if let Some(target) = self.closest_human(humans, config.zombie_vision_range) {
            let distance = (self.position - target.position).length();
            if distance > 0.01 {
                if distance < config.proximity_threshold {
                    target.is_infected = true;
                }

                let dir = (target.position - self.position).normalize();
                self.speed_current = dir * self.speed_chasing;
                #[cfg(debug_assertions)]
                println!("x:{} y:{}", self.position.x, self.position.y);
                #[cfg(debug_assertions)]
                println!("target x:{} y:{}", target.position.x, target.position.y);
            } else {
                self.speed_current = Vec2::new(0.0, 0.0);
            }
        } else {
            self.speed_current = self.speed;
        }

        self.position += self.speed_current;

        if self.position.x < 0.0 {
            self.speed_current.x = -self.speed_current.x;
            self.position.x = 0.0; // Asegura que la unidad no se salga por la izquierda
        } else if self.position.x > win_width {
            self.speed_current.x = -self.speed_current.x;
            self.position.x = win_width; // Asegura que la unidad no se salga por la derecha
        }

        if self.position.y < 0.0 {
            self.speed_current.y = -self.speed_current.y;
            self.position.y = 0.0; // Asegura que la unidad no se salga por arriba
        } else if self.position.y > win_height {
            self.speed_current.y = -self.speed_current.y;
            self.position.y = win_height; // Asegura que la unidad no se salga por abajo
        }
        assert!(
            !self.position.x.is_nan() && !self.position.y.is_nan(),
            "Position is NaN"
        );
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

    pub fn closest_human<'a>(&self, humans: &'a mut [Human], vision_range: f32) -> Option<&'a mut Human> {
        humans.iter_mut()
            .filter(|human| distance(self.position, human.position) < vision_range)
            .min_by(|a, b| {
                let dist_a = distance(self.position, a.position);
                let dist_b = distance(self.position, b.position);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}
