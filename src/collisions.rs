use glam::Vec2;

use crate::human::Human;
use crate::zombie::Zombie;
use crate::utils::distance;
use crate::quadtree::{Quadtree,Rectangle};


pub fn handle_collisions(humans: &mut Vec<Human>, zombies: &mut Vec<Zombie>, quadtree: Quadtree) {
    let mut human_adjustments = Vec::new();
    let mut zombie_adjustments = Vec::new();

    // Human-Human collisions
    for i in 0..humans.len() {
        let (left, right) = humans.split_at_mut(i + 1);
        if let Some(current_human) = left.last() {
            for (j, other_human) in right.iter().enumerate() {
                if let Some(adjustment) = resolve_collision(current_human, other_human) {
                    human_adjustments.push((i, adjustment));
                    human_adjustments.push((i + 1 + j, -adjustment)); // note the inverse adjustment for the other entity
                }
            }
        }
    }

    // Zombie-Zombie collisions
    for i in 0..zombies.len() {
        let current_zombie  = &zombies[i];

        let mut nearby_zombies = Vec::new();
        let range = Rectangle {
            x: current_zombie.position.x - current_zombie.radius(),
            y: current_zombie.position.y - current_zombie.radius(),
            w: 2.0 * current_zombie.radius(),
            h: 2.0 * current_zombie.radius()
        };
        quadtree.query(&range, &mut nearby_zombies);

        for other_zombie in nearby_zombies {
            if let Some(adjustment) = resolve_collision(current_zombie, other_zombie) {
                zombie_adjustments.push((i, adjustment));
                if let Some(other_index) = zombies.iter().position(|zomb: &Zombie| zomb == other_zombie) {
                    zombie_adjustments.push((other_index, -adjustment));
                }
            }
        }

    }

    // Aplica los ajustes para los humanos
    for (index, adjustment) in human_adjustments.iter() {
        humans[*index].adjust_position(*adjustment);
    }

    // Aplica los ajustes para los zombies
    for (index, adjustment) in zombie_adjustments.iter() {
        zombies[*index].adjust_position(*adjustment);
    }
}



fn resolve_collision<A: Collidable, B: Collidable>(entity_a: &A, entity_b: &B) -> Option<Vec2> {
    let dist = distance(entity_a.position(), entity_b.position());
    let radius_sum = entity_a.radius() + entity_b.radius();

    if dist < radius_sum {
        if dist == 0.0 {
            let random_direction = Vec2::new(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0).normalize();
            return Some(-0.1 * random_direction);
        }

        let overlap = 0.5 * (dist - radius_sum) + 1.0;
        let direction = (entity_b.position() - entity_a.position()).normalize();
        return Some(overlap * direction);
    }

    None
}

pub trait Collidable {
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