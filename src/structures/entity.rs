use crate::structures::{Rvector, Scaler};

pub trait Entity {
    fn get_mass(&self) -> Scaler;
    fn get_position(&self) -> Rvector;
    fn set_position(&mut self, position: Rvector);
    fn get_velocity(&self) -> Rvector;
    fn set_velocity(&mut self, velocity: Rvector);
    fn get_acceleration(&self) -> Rvector;
    fn set_acceleration(&mut self, acceleration: Rvector);
    fn get_force(&self) -> Rvector;
    fn add_force(&mut self, force: Rvector);
    fn get_delta_force(&self) -> Rvector;
    fn tick(&mut self, dt: Scaler) {
        let mass = self.get_mass();
        // a = kt; k = delta_a / delta_t
        let delta_acc = &self.get_delta_force() / &mass;
        // v = 1/2kt^2; delta_v = 1/2 delta_a / delta_t
        let delta_vel = &delta_acc * &( &dt / 2.0 );
        // s = 1/6kt^3; delta_s = 1/6 delta_a / delta_t^2  = 1/3 delta_v / delta_t
        let delta_pos = &delta_vel * &( &dt / 3.0 );
        self.set_position(self.get_position() + delta_pos);
        self.set_velocity(self.get_velocity() + delta_vel);
        self.set_acceleration(self.get_acceleration() + delta_acc);
    }
    fn kinetic_energy(&self) -> Scaler {
        let mass = self.get_mass();
        let vel = self.get_velocity();
        let vel2 = &vel * &vel;
        mass * vel2 / 2.0
    }
}