use crate::structures::Unit;

pub const SECOND: Unit = Unit { basic_units: [1, 0, 0, 0, 0, 0, 0] };
pub const METER: Unit = Unit { basic_units: [0, 1, 0, 0, 0, 0, 0] };
pub const KILOGRAM: Unit = Unit { basic_units: [0, 0, 1, 0, 0, 0, 0] };
pub const AMPERE: Unit = Unit { basic_units: [0, 0, 0, 1, 0, 0, 0] };
pub const KELVIN: Unit = Unit { basic_units: [0, 0, 0, 0, 1, 0, 0] };
pub const MOLE: Unit = Unit { basic_units: [0, 0, 0, 0, 0, 1, 0] };
pub const CANDELA: Unit = Unit { basic_units: [0, 0, 0, 0, 0, 0, 1] };
pub const V_UNIT: Unit = Unit { basic_units: [-1, 1, 0, 0, 0, 0, 0] };
pub const A_UNIT: Unit = Unit { basic_units: [-2, 1, 0, 0, 0, 0, 0] };
pub const COULOMB: Unit = Unit { basic_units: [1, 0, 0, 1, 0, 0, 0] };
pub const NEWTON: Unit = Unit { basic_units: [-2, 1, 1, 0, 0, 0, 0] };
pub const JOULE: Unit = Unit { basic_units: [-2, 2, 1, 0, 0, 0, 0] };