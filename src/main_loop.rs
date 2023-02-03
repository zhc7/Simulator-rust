use crate::entities::*;
use crate::structures::{A_UNIT, Entity, Scaler, SECOND};

const DT: Scaler = Scaler { val: 0.001, unit: SECOND };
const GRAVITY: Scaler = Scaler { val: 9.81, unit: A_UNIT };

pub fn run(entities: Vec<impl Entity>) {
    
}