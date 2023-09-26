use ggez::{self, graphics, Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::Color;
use glam::vec2;
use rand;

mod human;
use human::Human;

mod zombie;
use zombie::Zombie;

mod utils;

enum SimulationState {
    Running
}

struct Simulation {
    state: SimulationState,
    humans: Vec<Human>,
    zombies: Vec<Zombie>

}

impl EventHandler for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for human in &mut self.humans {
            human.update(_ctx);
        }

        for zombie in &mut self.zombies {
            zombie.update(_ctx, &self.humans);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        for human in &self.humans {
            human.draw(ctx)?;
     }

        for zombie in &self.zombies {
            zombie.draw(ctx)?;
        }

        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("zombie_simulator", "you");
    let (ctx, event_loop) = cb.build()?;
    
    let state = Simulation {
        state: SimulationState::Running,
        humans: (0..100).map(|_| Human {
            position: vec2(rand::random::<f32>() * 800.0, rand::random::<f32>() * 600.0),
            velocity: vec2(rand::random::<f32>() * 4.0 - 2.0, rand::random::<f32>() * 4.0 - 2.0),            
        }).collect(),
        zombies: (0..1).map(|_| Zombie {
            position: vec2(rand::random::<f32>() * 800.0, rand::random::<f32>() * 600.0),
            velocity: vec2(rand::random::<f32>() * 4.0 - 2.0, rand::random::<f32>() * 4.0 - 2.0),            
        }).collect(),
    };

    ggez::event::run(ctx, event_loop, state)
}
