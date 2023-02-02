use crate::structures::*;

pub struct Circle {
    pub radius: Scaler,
    pub pos: Rvector,
    pub mass: Scaler,
    pub vel: Rvector,
    pub acc: Rvector,
    pub force: Rvector,
}

impl Circle {
    pub fn new(radius: Scaler, pos: Rvector, mass: Scaler, vel: Option<Rvector>, acc: Option<Rvector>, force: Option<Rvector>) -> Circle {
        assert_eq!(radius.unit, Second ^ 1);
        let vel = match vel {
            Some(v) => v,
            None => Rvector::new(),
        };
        let acc = match acc {
            Some(a) => a,
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
            acc,
            force,
        }
    }
}

impl Entity for Circle {
    fn get_mass(&self) -> Scaler {
        self.mass.clone()
    }
    fn get_position(&self) -> Rvector {
        self.pos.clone()
    }
    fn set_position(&mut self, position: Rvector) {
        self.pos = position;
    }
    fn get_velocity(&self) -> Rvector {
        self.vel.clone()
    }
    fn set_velocity(&mut self, velocity: Rvector) {
        self.vel = velocity;
    }
    fn get_acceleration(&self) -> Rvector {
        self.acc.clone()
    }
    fn set_acceleration(&mut self, acceleration: Rvector) {
        self.acc = acceleration;
    }
    fn get_force(&self) -> Rvector {
        self.force.clone()
    }
    fn add_force(&mut self, force: Rvector) {
        self.force += force;
    }
}