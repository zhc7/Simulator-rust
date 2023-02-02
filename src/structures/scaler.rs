use crate::structures::{BasicUnit, Unit};

#[derive(Clone, Debug, PartialEq)]
pub struct Scaler {
    pub val: f64,
    pub unit: Unit,
}

impl Scaler {
    pub fn new(val: f64, unit: Option<BasicUnit>) -> Scaler {
        Scaler { val, unit: Unit::new(unit) }
    }
}