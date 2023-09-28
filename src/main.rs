use ggez::event::EventHandler;
use ggez::event::KeyCode;
use ggez::graphics::Color;
use ggez::input::keyboard;
use ggez::{self, conf, graphics, Context, GameResult};
use glam::{vec2, Vec2};
use rand;
use rand::seq::SliceRandom;

mod config;
mod hud;
mod human;
mod zombie;
mod utils;
mod collisions;

use collisions::handle_collisions;
use config::{load_config, Config};
use hud::HUD;
use human::{Human, HumanPersonalities, HumanState};
use zombie::Zombie;

enum SimulationState {
    Running,
    Paused,
}

struct Simulation {
    state: SimulationState,
    humans: Vec<Human>,
    zombies: Vec<Zombie>,
    config: Config,
    hud: HUD,
}

impl EventHandler for Simulation {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            SimulationState::Running => {
                if keyboard::is_key_pressed(ctx, KeyCode::P) {
                    self.state = SimulationState::Paused; // Cambia el estado de pausa.
                    return Ok(());
                }

                handle_collisions(&mut self.humans, &mut self.zombies);
                let delta_time = ggez::timer::delta(ctx).as_secs_f32();

                for human in &mut self.humans {
                    human.update(ctx, &self.zombies);
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
                        speed: Vec2 {
                            x: (self.config.zombie_speed),
                            y: (self.config.zombie_speed),
                        },
                        speed_chasing: Vec2 {
                            x: (self.config.zombie_speed_following),
                            y: (self.config.zombie_speed_following),
                        },
                    })
                    .collect();
                self.zombies.extend(new_zombies);

                self.hud.update(self.humans.len(), self.zombies.len());

                // Deleting the humans infected
                self.humans.retain(|h| !h.is_infected);
            }
            SimulationState::Paused => {
                if keyboard::is_key_pressed(ctx, KeyCode::P) {
                    self.state = SimulationState::Running; // Cambia el estado de pausa.
                }
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

        self.hud.draw(ctx)?;
        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let config_loaded = load_config().expect("Failed to load config");

    let cb = ggez::ContextBuilder::new("Zombie Simulator", "Mestre")
        .window_setup(conf::WindowSetup::default().title("Zombie Simulator"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(config_loaded.screen_width, config_loaded.screen_height),
        );
    let (ctx, event_loop) = cb.build()?;

    let state = Simulation {
        state: SimulationState::Running,
        humans: (0..config_loaded.humans)
            .map(|_| {
                let random_personality = *HumanPersonalities::VARIANTS
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                let random_state = *HumanState::VARIANTS
                    .choose(&mut rand::thread_rng())
                    .unwrap();

                Human {
                    position: vec2(
                        rand::random::<f32>() * config_loaded.screen_width,
                        rand::random::<f32>() * config_loaded.screen_height,
                    ),
                    speed: vec2(
                        rand::random::<f32>() * 4.0 - 2.0,
                        rand::random::<f32>() * 4.0 - 2.0,
                    ),
                    time_near_zombie: 0.0,
                    is_infected: false,
                    personality: random_personality,
                    state: random_state,
                }
            })
            .collect(),
        zombies: (0..config_loaded.zombies)
            .map(|_| Zombie {
                position: vec2(
                    rand::random::<f32>() * config_loaded.screen_width,
                    rand::random::<f32>() * config_loaded.screen_height,
                ),
                speed: vec2(config_loaded.zombie_speed, config_loaded.zombie_speed),
                speed_chasing: vec2(
                    config_loaded.zombie_speed_following,
                    config_loaded.zombie_speed_following,
                ),
            })
            .collect(),
        config: config_loaded,
        hud: HUD::new(),
    };

    ggez::event::run(ctx, event_loop, state)
}
