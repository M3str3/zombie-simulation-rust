use ggez::{self, Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::graphics::Color;
use glam::{vec2, Vec2};
use rand;

mod human;
mod zombie;
mod utils;
mod config;

use human::Human;
use zombie::Zombie;
use config::{load_config, Config};

const GAME_WIDTH: f32 = 800.0;
const GAME_HEIGHT: f32 = 600.0;

enum SimulationState {
    Running,
}

struct Simulation {
    state: SimulationState,
    humans: Vec<Human>,
    zombies: Vec<Zombie>,
    config: Config,
}

impl EventHandler for Simulation {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            SimulationState::Running => {
                let delta_time = ggez::timer::delta(ctx).as_secs_f32();

                for human in &mut self.humans {
                    human.update(ctx);
                }

                for zombie in &mut self.zombies {
                    zombie.update(ctx, &self.humans, &self.config);
                }

                // Infect humans
                for human in &mut self.humans {
                    let mut is_near_any_zombie = false;

                    for zombie in &self.zombies {
                        if utils::is_near(human, zombie, self.config.proximity_threshold) {
                            is_near_any_zombie = true;
                            human.time_near_zombie += delta_time;

                            if human.time_near_zombie >= 1.0
                                && rand::random::<f32>() < self.config.infection_rate
                            {
                                #[cfg(debug_assertions)]
                                println!("Human infected");
                                human.is_infected = true;
                            }
                            break; // Si el humano ya está cerca de un zombi, no hay necesidad de verificar los demás zombis
                        }
                    }

                    if !is_near_any_zombie {
                        human.time_near_zombie = 0.0;
                    }
                }
                // Conver humans with is_infected
                let new_zombies: Vec<Zombie> = self
                    .humans
                    .iter()
                    .filter(|h| h.is_infected)
                    .map(|h| Zombie {
                        position: h.position,
                        speed: h.velocity,
                        speed_chasing: Vec2 {
                            x: (self.config.zombie_speed_following),
                            y: (self.config.zombie_speed_following),
                        },
                    })
                    .collect();
                self.zombies.extend(new_zombies);

                // Deleting the humans infected
                self.humans.retain(|h| !h.is_infected);
            }
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
    let config_loaded = load_config().expect("Failed to load config");
    let cb = ggez::ContextBuilder::new("zombie_simulator", "you");
    let (ctx, event_loop) = cb.build()?;

    let state = Simulation {
        state: SimulationState::Running,
        humans: (0..100)
            .map(|_| Human {
                position: vec2(rand::random::<f32>() * GAME_WIDTH, rand::random::<f32>() * GAME_HEIGHT),
                velocity: vec2(
                    rand::random::<f32>() * 4.0 - 2.0,
                    rand::random::<f32>() * 4.0 - 2.0,
                ),
                time_near_zombie: 0.0,
                is_infected: false,
            })
            .collect(),
        zombies: (0..config_loaded.zombies)
            .map(|_| Zombie {
                position: vec2(rand::random::<f32>() * GAME_WIDTH, rand::random::<f32>() * GAME_HEIGHT),
                speed: vec2(config_loaded.zombie_speed, config_loaded.zombie_speed),
                speed_chasing: vec2(
                    config_loaded.zombie_speed_following,
                    config_loaded.zombie_speed_following,
                ),
            })
            .collect(),
        config: config_loaded,
    };

    ggez::event::run(ctx, event_loop, state)
}
