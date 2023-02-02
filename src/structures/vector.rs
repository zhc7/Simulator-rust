use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};

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
}

impl Index<usize> for Rvector {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.val[index]
    }
}

impl Add for Rvector {
    type Output = Rvector;

    fn add(self, other: Rvector) -> Rvector {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        let mut f = self.clone();
        for i in 0..DIM {
            f.val[i] += other.val[i];
        }
        f
    }
}

impl AddAssign for Rvector {
    fn add_assign(&mut self, other: Rvector) {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        for i in 0..DIM {
            self.val[i] += other.val[i];
        }
    }
}

impl Sub for Rvector {
    type Output = Rvector;

    fn sub(self, other: Rvector) -> Rvector {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        let mut f = self.clone();
        for i in 0..DIM {
            f.val[i] -= other.val[i];
        }
        f
    }
}

impl SubAssign for Rvector {
    fn sub_assign(&mut self, other: Rvector) {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        for i in 0..DIM {
            self.val[i] -= other.val[i];
        }
    }
}

impl Mul<Scaler> for Rvector {
    type Output = Rvector;

    fn mul(self, other: Scaler) -> Rvector {
        let mut f = self.clone();
        for i in 0..DIM {
            f.val[i] *= other.val;
        }
        f.unit *= other.unit;
        f
    }
}

impl MulAssign<Scaler> for Rvector {
    fn mul_assign(&mut self, other: Scaler) {
        for i in 0..DIM {
            self.val[i] *= other.val;
        }
        self.unit *= other.unit;
    }
}

impl Div<Scaler> for Rvector {
    type Output = Rvector;

    fn div(self, other: Scaler) -> Rvector {
        let mut f = self.clone();
        for i in 0..DIM {
            f.val[i] /= other.val;
        }
        f.unit /= other.unit;
        f
    }
}

impl DivAssign<Scaler> for Rvector {
    fn div_assign(&mut self, other: Scaler) {
        for i in 0..DIM {
            self.val[i] /= other.val;
        }
        self.unit /= other.unit;
    }
}