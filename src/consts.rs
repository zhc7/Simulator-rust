use crate::structures::{Scaler, Unit};

pub const EPSILON0: Scaler = Scaler { val: 8.854187817620389e-12, unit: Unit { basic_units: [4, -3, -1, 2, 0, 0, 0] } };
pub const KE: Scaler = Scaler { val: 8.9875517873681764e9, unit: Unit { basic_units: [-4, 3, 1, -2, 0, 0, 0] } };
pub const PI: Scaler = Scaler { val: std::f64::consts::PI, unit: Unit { basic_units: [0, 0, 0, 0, 0, 0, 0] } };
pub const E: Scaler = Scaler { val: std::f64::consts::E, unit: Unit { basic_units: [0, 0, 0, 0, 0, 0, 0] } };