use crate::consts::*;
use crate::structures::{Entity, Rvector, Scaler};

pub struct Field {
    pub electric: ElectricField,
}

impl Field {
    pub fn get_force(&self, entity: &Box<dyn Entity>) -> Rvector {
        self.electric.get_force(entity)
    }
    pub fn get_potential_energy(&self, entity: &Box<dyn Entity>) -> Scaler {
        self.electric.get_potential_energy(entity)
    }
}

pub struct ElectricField {
    pub center_charge: Scaler,
    pub center_position: Rvector,
}

impl ElectricField {
    pub fn get_force(&self, entity: &Box<dyn Entity>) -> Rvector {
        let charge = entity.get_charge();
        let position = entity.get_position();
        let distance = &position - &self.center_position;
        let distance2 = &distance * &distance;
        let distance3 = distance2 * &distance;
        let force = &KE * &distance * (&charge * &self.center_charge / &distance3.length());
        force
    }
    pub fn get_potential_energy(&self, entity: &Box<dyn Entity>) -> Scaler {
        let charge = entity.get_charge();
        let position = entity.get_position();
        let distance = &self.center_position - &position;
        let potential = &KE * &charge * &self.center_charge / distance.length();
        potential
    }
}