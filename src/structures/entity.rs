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
    fn tick(&mut self, dt: Scaler) {
        let mass = self.get_mass();
        self.set_position(self.get_velocity() * dt.clone());
        self.set_velocity(self.get_acceleration() * dt);
        self.set_acceleration(self.get_force() / mass);
    }
}