use glam::Vec2;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};

use crate::config::Config;
use crate::utils::{vec2_to_point2, is_near};
use crate::zombie::Zombie;

#[derive(Copy, Clone)]
pub enum HumanState {
    Normal,     // Estado normal, se comporta según su personalidad.
    Panicking,  // Ha estado cerca de un zombi y está en pánico (podría moverse erráticamente).
    Hiding,     // Intenta esconderse de los zombis (podría buscar obstáculos o edificios).
    InGroup,    // Se ha agrupado con otros humanos para seguridad.
}

impl HumanState {
    pub const VARIANTS: &'static [HumanState] = &[
        HumanState::Normal,
        HumanState::Panicking,
        HumanState::Hiding,
        HumanState::InGroup,
    ];
}
#[derive(Copy, Clone)]
pub enum HumanPersonalities {
    Cowardly,   // Evita a los zombies a toda costa, prioriza escapar sobre cualquier otra acción.
    Brave,      // Puede intentar enfrentar o distraer al zombi.
   Indifferent,// Sigue su camino sin prestar demasiada atención.
}

impl HumanPersonalities {
    pub const VARIANTS: &'static [HumanPersonalities] = &[
        HumanPersonalities::Cowardly,
        HumanPersonalities::Brave,
        HumanPersonalities::Indifferent,
    ];
}

pub struct Human {
    pub position: Vec2,
    pub speed: Vec2,
    pub time_near_zombie: f32,
    pub is_infected: bool,
    pub personality: HumanPersonalities,
    pub state: HumanState
}

impl Human {

    pub fn update(&mut self, ctx: &mut Context,zombies: &[Zombie],humans: &[Human],config: &Config) {
        let (win_width, win_height) = graphics::drawable_size(ctx);

        match self.state {
            HumanState::Normal => {
                self.normal_behavior();
            },
            HumanState::Panicking => {
                self.panicking_behavior();
            },
            HumanState::Hiding => {
                self.hiding_behavior();
            },
            HumanState::InGroup => {
                self.group_behavior(humans, config);
            },
        }

        
        for zombie in zombies {
            if is_near(self, zombie, config.humans_zombie_distance_to_start_run) {  // por ejemplo, 50.0 es nuestra distancia límite
                self.run_from(zombie);
                break;  // Rompemos después de encontrar el primer zombi cercano
            }
        }
        self.state = HumanState::InGroup;
        self.position += self.speed;

        if self.position.x < 0.0 {
            self.speed.x = -self.speed.x;
            self.position.x = 0.0; // Asegura que la unidad no se salga por la izquierda
        } else if self.position.x > win_width {
            self.speed.x = -self.speed.x;
            self.position.x = win_width; // Asegura que la unidad no se salga por la derecha
        }
        
        if self.position.y < 0.0 {
            self.speed.y = -self.speed.y;
            self.position.y = 0.0; // Asegura que la unidad no se salga por arriba
        } else if self.position.y > win_height {
            self.speed.y = -self.speed.y;
            self.position.y = win_height; // Asegura que la unidad no se salga por abajo
        }
    }


    fn normal_behavior(&mut self) {
        match self.personality {
            HumanPersonalities::Cowardly => {
                // Aquí puedes agregar lógica específica, por ejemplo, alejarse más rápidamente de los zombis.
            },
            HumanPersonalities::Brave => {
                // Podría intentar acercarse a los zombis o mantener su posición.
            },
            HumanPersonalities::Indifferent => {
                // Simplemente sigue su camino.
            },
        }
    }

    fn panicking_behavior(&mut self) {
        // Aquí, por ejemplo, podrías hacer que el humano se mueva erráticamente.
        // O aumentar su velocidad temporalmente.

    }

    fn hiding_behavior(&mut self) {
        // Aquí, podrías hacer que el humano busque edificios cercanos u obstáculos para esconderse.
    }

    fn group_behavior(&mut self, humans: &[Human], config: &Config) {
        let mut nearby_humans: Vec<Vec2> = Vec::new();
        for human in humans {
            // No te querrás agrupar contigo mismo.
            if (self.position - human.position).length() > 0.0 {
                // Aquí estamos suponiendo que hay una cierta distancia para considerar a otros humanos "cercanos".
                // Puede ajustar el valor según sea necesario.
                if is_near(self, human, config.human_grouping_distance) {
                    nearby_humans.push(human.position);
                }
            }
        }
    
        if !nearby_humans.is_empty() {
            // Calcula el punto medio de todos los humanos cercanos.
            let mut average_position = Vec2::new(0.0, 0.0);
            for position in &nearby_humans {
                average_position += *position;
            }
            average_position /= nearby_humans.len() as f32;
    
            // Dirige al humano actual hacia ese punto medio.
            let direction = (average_position - self.position).normalize();
            self.speed = direction * self.speed.length();
        }
    }

    fn run_from(&mut self, zombie: &Zombie) {
        let run_direction = (self.position - zombie.position).normalize();
        self.speed = run_direction * self.speed.length();
    }


    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let color = if self.is_infected {
            Color::YELLOW 
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
