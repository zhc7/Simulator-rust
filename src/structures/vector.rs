use std::ops::Index;
use std::ops;

use crate::structures::{DIM, Scaler, Unit};

#[derive(Clone, Debug)]
pub struct Rvector {
    pub val: [f64; DIM],
    pub unit: Unit,
}

impl Rvector {
    pub fn new() -> Rvector {
        Rvector { val: [0.0; DIM], unit: Unit::new(None) }
    }

    pub fn length(&self) -> Scaler {
        let mut sum:f64 = 0.0;
        for i in 0..DIM {
            sum += self.val[i] * self.val[i];
        }
        Scaler { val: sum.sqrt(), unit: self.unit }
    }
}

impl Index<usize> for Rvector {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.val[index]
    }
}

impl_op_ex!(+ |a: &Rvector, b: &Rvector| -> Rvector {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    let mut f = a.clone();
    for i in 0..DIM {
        f.val[i] += b.val[i];
    }
    f
});

impl_op_ex!(+= |a: &mut Rvector, b: &Rvector| {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    for i in 0..DIM {
        a.val[i] += b.val[i];
    }
});

impl_op_ex!(- |a: &Rvector, b: &Rvector| -> Rvector {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    let mut f = a.clone();
    for i in 0..DIM {
        f.val[i] -= b.val[i];
    }
    f
});

impl_op_ex!(-= |a: &mut Rvector, b: &Rvector| {
    assert_eq!(a.unit, b.unit, "Unit mismatch: {} != {}", a.unit, b.unit);
    for i in 0..DIM {
        a.val[i] -= b.val[i];
    }
});

impl_op_ex_commutative!(* |a: &Rvector, b: &Scaler| -> Rvector {
    let mut f = a.clone();
    for i in 0..DIM {
        f.val[i] *= b.val;
    }
    f
});

impl_op_ex!(*= |a: &mut Rvector, b: &Scaler| {
    for i in 0..DIM {
        a.val[i] *= b.val;
    }
});

impl_op_ex!(/ |a: &Rvector, b: &Scaler| -> Rvector {
    let mut f = a.clone();
    for i in 0..DIM {
        f.val[i] /= b.val;
    }
    f
});

impl_op_ex!(/= |a: &mut Rvector, b: &Scaler| {
    for i in 0..DIM {
        a.val[i] /= b.val;
    }
});

impl_op_ex!(* |a: &Rvector, b: &Rvector| -> Scaler {
    let mut f = Scaler { val: 0.0, unit: a.unit * b.unit };
    for i in 0..DIM {
        f.val += a.val[i] * b.val[i];
    }
    f
});