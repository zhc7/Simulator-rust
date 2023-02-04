use crate::structures::*;

pub struct Circle {
    pub radius: Scaler,
    pub pos: Rvector,
    pub mass: Scaler,
    pub charge: Scaler,
    pub vel: Rvector,
    pub acc: Rvector,
    pub old_force: Rvector,
    pub new_force: Rvector,
}

impl Circle {
    pub fn new(radius: Scaler, pos: Rvector, mass: Scaler, charge: Option<Scaler>, vel: Option<Rvector>, acc: Option<Rvector>, force: Option<Rvector>) -> Circle {
        assert_eq!(radius.unit, METER, "Unit mismatch: {} != {}", radius.unit, METER);
        assert_eq!(pos.unit, METER, "Unit mismatch: {} != {}", pos.unit, METER);
        assert_eq!(mass.unit, KILOGRAM, "Unit mismatch: {} != {}", mass.unit, KILOGRAM);
        let charge = match charge {
            Some(c) => {
                assert_eq!(c.unit, COULOMB, "Unit mismatch: {} != {}", c.unit, COULOMB);
                c
            }
            None => Scaler::zero(COULOMB),
        };
        let vel = match vel {
            Some(v) => {
                assert_eq!(v.unit, V_UNIT, "Unit mismatch: {} != {}", v.unit, V_UNIT);
                v
            }
            None => Rvector::zero(V_UNIT),
        };
        let acc = match acc {
            Some(a) => {
                assert_eq!(a.unit, A_UNIT, "Unit mismatch: {} != {}", a.unit, A_UNIT);
                a
            }
            None => Rvector::zero(A_UNIT),
        };
        let force = match force {
            Some(f) => {
                assert_eq!(f.unit, NEWTON, "Unit mismatch: {} != {}", f.unit, NEWTON);
                f
            }
            None => Rvector::zero(NEWTON),
        };
        Circle {
            radius,
            pos,
            mass,
            charge,
            vel,
            acc,
            old_force: force,
            new_force: Rvector::zero(NEWTON),
        }
    }
}

impl Entity for Circle {
    fn get_mass(&self) -> Scaler {
        self.mass.clone()
    }
    fn get_charge(&self) -> Scaler {
        self.charge.clone()
    }
    fn set_charge(&mut self, charge: Scaler) {
        assert_eq!(charge.unit, COULOMB, "Unit mismatch: {} != {}", charge.unit, COULOMB);
        self.charge = charge;
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
        self.new_force.clone()
    }
    fn add_force(&mut self, force: Rvector) {
        assert_eq!(force.unit, NEWTON, "Unit mismatch: {} != {}", force.unit, NEWTON);
        self.new_force += force;
    }
    fn get_delta_force(&self) -> Rvector {
        &self.new_force - &self.old_force
    }
    fn clear_delta_force(&mut self) {
        self.old_force = self.new_force.clone();
        self.new_force = Rvector::zero(NEWTON);
    }
}