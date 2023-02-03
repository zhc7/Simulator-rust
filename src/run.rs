use crate::entities::*;
use crate::structures::{A_UNIT, Entity, Scaler, SECOND};

const DT: Scaler = Scaler { val: 0.001, unit: SECOND };

pub fn run(entities: &mut Vec<impl Entity>, time: Scaler) {
    let mut t = Scaler { val: 0.0, unit: SECOND };
    while t < time {
        for i in 0..entities.len() {
            let field = entities[i].get_field();
            for j in 0..entities.len() {
                let other = &mut entities[j];
                if i != j {
                    let force = field.get_force(other);
                    other.add_force(force);
                }
            }
        }
        for i in 0..entities.len() {
            let entity = &mut entities[i];
            entity.tick(DT);
        }
        t += DT;
    }
}