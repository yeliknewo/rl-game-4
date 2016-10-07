use dependencies::specs::{Component, VecStorage};
use dependencies::cgmath::{Vector2};
use utils::{Coord};

pub struct CompMoving {
    velocity: Vector2<Coord>,
}

impl CompMoving {
    pub fn new(velocity: Vector2<Coord>) -> CompMoving {
        CompMoving {
            velocity: velocity,
        }
    }

    pub fn get_velocity(&self) -> &Vector2<Coord> {
        &self.velocity
    }

    pub fn get_mut_velocity(&mut self) -> &mut Vector2<Coord> {
        &mut self.velocity
    }
}

impl Component for CompMoving {
    type Storage = VecStorage<CompMoving>;
}
