use crate::structure::*;


pub struct Circle {
    pub radius: f64,
    pub pos: Rvector,
    pub mass: f64,
    pub vel: Rvector,
    pub force: Rvector,
}

impl Circle {
    pub fn new(radius: f64, pos: Rvector, mass: f64, vel: Option<Rvector>, force: Option<Rvector>) -> Circle {
        let vel = match vel {
            Some(v) => v,
            None => Rvector::new(),
        };
        let force = match force {
            Some(f) => f,
            None => Rvector::new(),
        };
        Circle {
            radius,
            pos,
            mass,
            vel,
            force,
        }
    }
}

impl Entity for Circle {
    fn get_mass(&self) -> f64 {
        self.mass
    }
    fn get_position(&self) -> Rvector {
        self.pos.clone()
    }
    fn get_velocity(&self) -> Rvector {
        self.vel.clone()
    }
    fn get_force(&self) -> Rvector {
        self.force.clone()
    }
    fn add_force(&mut self, force: Rvector) {
        self.force += force;
    }
    fn tick(&mut self, dt: f64) {
        todo!()
    }
}