use ggez::graphics::{self, Color, Text};
use ggez::{Context, GameResult};

pub struct HUD {
    pub num_humans: usize,
    pub num_zombies: usize,
}

impl HUD {
    pub fn new() -> Self {
        Self {
            num_humans: 0,
            num_zombies: 0
        }
    }

    pub fn update(&mut self, num_humans: usize, num_zombies: usize) {
        self.num_humans = num_humans;
        self.num_zombies = num_zombies;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let text = Text::new(format!(
            "Humans: {} | Zombies: {} ",
            self.num_humans, self.num_zombies
        ));
        
        graphics::draw(ctx, &text, (ggez::mint::Point2 { x: 10.0, y: 10.0 }, Color::BLACK))?;
        Ok(())
    }
}
