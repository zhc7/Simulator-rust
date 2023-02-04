use crate::structures::{COULOMB, ElectricField, Field, Rvector, Scaler};

pub trait Entity {
    fn get_mass(&self) -> Scaler;
    fn get_charge(&self) -> Scaler {
        Scaler { val: 0.0, unit: COULOMB }
    }
    fn set_charge(&mut self, charge: Scaler) {
        assert_eq!(charge.unit, COULOMB, "Unit mismatch: {} != {}", charge.unit, COULOMB);
    }
    fn get_position(&self) -> Rvector;
    fn set_position(&mut self, position: Rvector);
    fn get_velocity(&self) -> Rvector;
    fn set_velocity(&mut self, velocity: Rvector);
    fn get_acceleration(&self) -> Rvector;
    fn set_acceleration(&mut self, acceleration: Rvector);
    fn get_force(&self) -> Rvector;
    fn add_force(&mut self, force: Rvector);
    fn get_delta_force(&self) -> Rvector;
    fn clear_delta_force(&mut self);
    fn get_field(&self) -> Field {
        Field {
            electric: ElectricField {
                center_charge: self.get_charge(),
                center_position: self.get_position(),
            },
        }
    }
    fn tick(&mut self, dt: Scaler) {
        let mass = self.get_mass();
        // a = a0 + kt; k = delta_a / delta_t; delta_a = k * delta_t;
        let delta_acc = &self.get_delta_force() / &mass;
        // v = v0 + a0t + 1/2kt^2; delta_v = a0 * delta_t + 1/2 * delta_a * delta_t;
        let delta_vel = (self.get_acceleration() + &delta_acc / 2.0) * &dt;
        // x = x0 + v0t + 1/2a0t^2 + 1/6kt^3;
        // delta_x = v0 * delta_t + 1/2 * a0 * delta_t^2 + 1/6 * delta_a * delta_t^2
        //         = ( v0 + 1/2 * delta_v - 1/12 * delta_a * delta_t ) * delta_t
        let delta_pos = (self.get_velocity() + &delta_vel / 2.0 - &delta_acc * &dt / 12.0) * dt;
        self.set_position(self.get_position() + delta_pos);
        self.set_velocity(self.get_velocity() + delta_vel);
        self.set_acceleration(self.get_acceleration() + delta_acc);
        self.clear_delta_force();
    }
    fn get_kinetic_energy(&self) -> Scaler {
        let mass = self.get_mass();
        let vel = self.get_velocity();
        let vel2 = &vel * &vel;
        mass * vel2 / 2.0
    }
    fn draw(&self);
}