use std::cmp::Ordering;
use crate::structures::{Unit};
use std::ops;

#[derive(Clone, Debug, PartialEq)]
pub struct Scaler {
    pub val: f64,
    pub unit: Unit,
}

impl Scaler {
    pub fn new(val: f64, unit: Option<Unit>) -> Scaler {
        Scaler {
            val,
            unit: match unit {
            Some(unit) => unit,
            None => Unit::new(None),
        }
    }
    }
}

impl_op_ex!(+ |a: &Scaler, b: &Scaler| -> Scaler {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    Scaler { val: a.val + b.val, unit: a.unit}
});

impl_op_ex!(- |a: &Scaler, b: &Scaler| -> Scaler {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    Scaler { val: a.val - b.val, unit: a.unit}
});

impl_op_ex!(* |a: &Scaler, b: &Scaler| -> Scaler {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    Scaler { val: a.val * b.val, unit: a.unit * b.unit}
});

impl_op_ex!(/ |a: &Scaler, b: &Scaler| -> Scaler {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    Scaler { val: a.val / b.val, unit: a.unit / b.unit}
});

impl_op_ex!(+= |a: &mut Scaler, b: &Scaler| {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    a.val += b.val;
});

impl_op_ex!(-= |a: &mut Scaler, b: &Scaler| {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    a.val -= b.val;
});

impl_op_ex!(*= |a: &mut Scaler, b: &Scaler| {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    a.val *= b.val;
    a.unit *= b.unit;
});

impl_op_ex!(/= |a: &mut Scaler, b: &Scaler| {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    a.val /= b.val;
    a.unit /= b.unit;
});

impl_op_ex_commutative!(* |a: &Scaler, b: &f64| -> Scaler {
    Scaler { val: a.val * b, unit: a.unit.clone()}
});

impl_op_ex_commutative!(/ |a: &Scaler, b: &f64| -> Scaler {
    Scaler { val: a.val / b, unit: a.unit.clone()}
});

impl_op_ex!(*= |a: &mut Scaler, b: &f64| {
    a.val *= b;
});

impl_op_ex!(/= |a: &mut Scaler, b: &f64| {
    a.val /= b;
});

impl PartialOrd for Scaler {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        self.val.partial_cmp(&other.val)
    }
}

impl Eq for Scaler {}

impl Ord for Scaler {
    fn cmp(&self, other: &Self) -> Ordering {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        self.val.partial_cmp(&other.val).unwrap()
    }
}