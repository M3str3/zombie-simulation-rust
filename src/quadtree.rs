use glam::Vec2;
use crate::{zombie::Zombie, collisions::Collidable};

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}


impl Rectangle {
    pub fn contains(&self, point: &Vec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.w &&
        point.y >= self.y && point.y <= self.y + self.h
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        !(self.x + self.w < other.x || 
          self.x > other.x + other.w ||
          self.y + self.h < other.y ||
          self.y > other.y + other.h)
    }
}


pub struct Quadtree {
    boundary: Rectangle,
    zombies: Vec<Zombie>,
    nw: Option<Box<Quadtree>>,
    ne: Option<Box<Quadtree>>,
    sw: Option<Box<Quadtree>>,
    se: Option<Box<Quadtree>>,
    capacity: usize,
}

impl Quadtree {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        Self {
            boundary,
            zombies: Vec::with_capacity(capacity),
            nw: None,
            ne: None,
            sw: None,
            se: None,
            capacity,
        }
    }
    
    pub fn insert(&mut self,zombie: Zombie) -> bool {
        if !self.boundary.contains(&zombie.position()){
            return false;
        }

        if self.zombies.len() < self.capacity {
            self.zombies.push(zombie);
            return true;
        }

        if self.nw.is_none() {
            self.subdivide();
        }

        if self.nw.as_mut().unwrap().insert(zombie.clone()) { return true; }
        if self.ne.as_mut().unwrap().insert(zombie.clone()) { return true; }
        if self.sw.as_mut().unwrap().insert(zombie.clone()) { return true; }
        if self.se.as_mut().unwrap().insert(zombie.clone()) { return true; }

        false
    }


    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w / 2.0;
        let h = self.boundary.h / 2.0;

        let nw_boundary = Rectangle { x: x, y: y, w: w, h: h };
        self.nw = Some(Box::new(Quadtree::new(nw_boundary, self.capacity)));

        let ne_boundary = Rectangle { x: x + w, y: y, w: w, h: h };
        self.ne = Some(Box::new(Quadtree::new(ne_boundary, self.capacity)));

        let sw_boundary = Rectangle { x: x, y: y + h, w: w, h: h };
        self.sw = Some(Box::new(Quadtree::new(sw_boundary, self.capacity)));

        let se_boundary = Rectangle { x: x + w, y: y + h, w: w, h: h };
        self.se = Some(Box::new(Quadtree::new(se_boundary, self.capacity)));
    }

    pub fn query<'a>(&'a self, range: &Rectangle, found: &mut Vec<&'a Zombie>) {
        if !self.boundary.intersects(range) {
            return;
        }

        for zombie in &self.zombies {
            if range.contains(&zombie.position()) {
                found.push(zombie);
            }
        }

        if self.nw.is_none() {
            return;
        }

        self.nw.as_ref().unwrap().query(range, found);
        self.ne.as_ref().unwrap().query(range, found);
        self.sw.as_ref().unwrap().query(range, found);
        self.se.as_ref().unwrap().query(range, found);
    }
}