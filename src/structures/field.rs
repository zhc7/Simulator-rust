use crate::structures::{Entity, Rvector, Scaler};
use crate::consts::*;

pub trait Field {
    fn get_force(&self, entity: &impl Entity) -> Rvector;
    fn get_potential(&self, entity: &impl Entity) -> Scaler;
}

pub struct ElectricField {
    pub center_charge: Scaler,
    pub center_position: Rvector,
}

impl Field for ElectricField {
    fn get_force(&self, entity: &impl Entity) -> Rvector {
        let charge = entity.get_charge();
        let position = entity.get_position();
        let distance = &self.center_position - &position;
        let distance2 = &distance * &distance;
        let force = &KE * &distance * ( &charge * &self.center_charge / &distance2 );
        force
    }
    fn get_potential(&self, entity: &impl Entity) -> Scaler {
        let charge = entity.get_charge();
        let position = entity.get_position();
        let distance = &self.center_position - &position;
        let potential = &KE * &charge * &self.center_charge / distance.length();
        potential
    }
}