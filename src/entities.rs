use crate::structures::*;

pub struct Circle {
    pub radius: Scaler,
    pub pos: Rvector,
    pub mass: Scaler,
    pub vel: Rvector,
    pub acc: Rvector,
    pub force: Rvector,
    pub delta_force: Rvector,
}

impl Circle {
    pub fn new(radius: Scaler, pos: Rvector, mass: Scaler, vel: Option<Rvector>, acc: Option<Rvector>, force: Option<Rvector>) -> Circle {
        assert_eq!(radius.unit, METER, "Unit mismatch: {} != {}", radius.unit, METER);
        assert_eq!(pos.unit, METER, "Unit mismatch: {} != {}", pos.unit, METER);
        assert_eq!(mass.unit, KILOGRAM, "Unit mismatch: {} != {}", mass.unit, KILOGRAM);
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
            delta_force: Rvector::new(),
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
        assert_eq!(position.unit, METER, "Unit mismatch: {} != {}", position.unit, METER);
        self.pos = position;
    }
    fn get_velocity(&self) -> Rvector {
        self.vel.clone()
    }
    fn set_velocity(&mut self, velocity: Rvector) {
        assert_eq!(velocity.unit, V_UNIT, "Unit mismatch: {} != {}", velocity.unit, V_UNIT);
        self.vel = velocity;
    }
    fn get_acceleration(&self) -> Rvector {
        self.acc.clone()
    }
    fn set_acceleration(&mut self, acceleration: Rvector) {
        assert_eq!(acceleration.unit, A_UNIT, "Unit mismatch: {} != {}", acceleration.unit, A_UNIT);
        self.acc = acceleration;
    }
    fn get_force(&self) -> Rvector {
        self.force.clone()
    }
    fn add_force(&mut self, force: Rvector) {
        assert_eq!(force.unit, NEWTON, "Unit mismatch: {} != {}", force.unit, NEWTON);
        self.delta_force += &force;
        self.force += force;
    }
    fn get_delta_force(&self) -> Rvector {
        self.delta_force.clone()
    }
}