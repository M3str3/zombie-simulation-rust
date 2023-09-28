use glam::Vec2;

use crate::human::Human;
use crate::zombie::Zombie;
use crate::utils::distance;

pub fn handle_collisions(humans: &mut Vec<Human>, zombies: &mut Vec<Zombie>) {
    // Humnas
    for i in 0..humans.len() {
        let (left, right) = humans.split_at_mut(i+1);
        if let Some(current_human) = left.last_mut() {
            for other_human in right.iter_mut() {
                resolve_collision(current_human, other_human);
            }
        }
    }

    // Zombies
    for i in 0..zombies.len() {
        let (left, right) = zombies.split_at_mut(i+1);
        if let Some(current_zombie) = left.last_mut() {
            for other_zombie in right.iter_mut() {
                resolve_collision(current_zombie, other_zombie);
            }
        }
    }

    // Humans with zombies
    for human in humans.iter_mut() {
        for zombie in zombies.iter_mut() {
            resolve_collision(human, zombie);
        }
    }
}

fn resolve_collision<A: Collidable, B: Collidable>(entity_a: &mut A, entity_b: &mut B) {
    let dist = distance(entity_a.position(), entity_b.position());  // Añadimos paréntesis aquí
    let radius_sum = entity_a.radius() + entity_b.radius();         // Y aquí también

    if dist < radius_sum {
        let overlap = 0.5 * (dist - radius_sum);
        let direction = (entity_b.position() - entity_a.position()).normalize();  // Y aquí

        entity_a.adjust_position(overlap * direction);
        entity_b.adjust_position(overlap * direction);
    }
}

trait Collidable {
    fn position(&self) -> Vec2;
    fn set_position(&mut self, new_pos: Vec2);
    fn adjust_position(&mut self, adjustment: Vec2) {
        let current = self.position();
        self.set_position(current + adjustment);
    }
    fn radius(&self) -> f32;
}

impl Collidable for Human {
    fn position(&self) -> Vec2 {
        self.position
    }

    fn radius(&self) -> f32 {
        5.0  
    }

    fn set_position(&mut self, new_pos: Vec2) {
        self.position = new_pos;
    }

    fn adjust_position(&mut self, adjustment: Vec2) {
        let current = self.position();
        self.set_position(current + adjustment);
    }
}

impl Collidable for Zombie {
    fn position(&self) -> Vec2 {
        self.position
    }

    fn radius(&self) -> f32 {
        5.0 
    }

    fn set_position(&mut self, new_pos: Vec2) {
        self.position = new_pos;
    }

    fn adjust_position(&mut self, adjustment: Vec2) {
        let current = self.position();
        self.set_position(current + adjustment);
    }
}