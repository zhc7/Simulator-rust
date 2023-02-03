use std::ops::{Mul, MulAssign, Div, DivAssign, Index, Add, AddAssign, Sub, SubAssign};
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

impl Mul for Scaler {
    type Output = Scaler;

    fn mul(self, other: Scaler) -> Scaler {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        Scaler { val: self.val * other.val, unit: self.unit * other.unit }
    }
}

impl MulAssign for Scaler {
    fn mul_assign(&mut self, other: Scaler) {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        self.val *= other.val;
        self.unit *= other.unit;
    }
}

impl Mul<f64> for Scaler {
    type Output = Scaler;

    fn mul(self, other: f64) -> Scaler {
        Scaler { val: self.val * other, unit: self.unit }
    }
}

impl MulAssign<f64> for Scaler {
    fn mul_assign(&mut self, other: f64) {
        self.val *= other;
    }
}

impl Mul<Scaler> for f64 {
    type Output = Scaler;

    fn mul(self, other: Scaler) -> Scaler {
        Scaler { val: self * other.val, unit: other.unit }
    }
}

impl Div for Scaler {
    type Output = Scaler;

    fn div(self, other: Scaler) -> Scaler {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        Scaler { val: self.val / other.val, unit: self.unit.clone() / other.unit.clone() }
    }
}

impl DivAssign for Scaler {
    fn div_assign(&mut self, other: Scaler) {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        self.val /= other.val;
        self.unit /= other.unit;
    }
}

impl Div<f64> for Scaler {
    type Output = Scaler;

    fn div(self, other: f64) -> Scaler {
        Scaler { val: self.val / other, unit: self.unit.clone() }
    }
}

impl DivAssign<f64> for Scaler {
    fn div_assign(&mut self, other: f64) {
        self.val /= other;
    }
}

impl Div<Scaler> for f64 {
    type Output = Scaler;

    fn div(self, other: Scaler) -> Scaler {
        Scaler { val: self / other.val, unit: other.unit }
    }
}

// for &

impl Mul for &Scaler {
    type Output = Scaler;

    fn mul(self, other: &Scaler) -> Scaler {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        Scaler { val: self.val * other.val, unit: self.unit * other.unit }
    }
}

impl MulAssign<&Scaler> for Scaler {
    fn mul_assign(&mut self, other: &Scaler) {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        self.val *= other.val;
        self.unit *= other.unit;
    }
}

impl Mul<f64> for &Scaler {
    type Output = Scaler;

    fn mul(self, other: f64) -> Scaler {
        Scaler { val: self.val * other, unit: self.unit }
    }
}

impl Mul<&Scaler> for f64 {
    type Output = Scaler;

    fn mul(self, other: &Scaler) -> Scaler {
        Scaler { val: self * other.val, unit: other.unit }
    }
}

impl Div for &Scaler {
    type Output = Scaler;

    fn div(self, other: &Scaler) -> Scaler {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        Scaler { val: self.val / other.val, unit: self.unit.clone() / other.unit.clone() }
    }
}

impl DivAssign<&Scaler> for Scaler {
    fn div_assign(&mut self, other: &Scaler) {
        assert_eq!(self.unit, other.unit, "Unit mismatch: {} != {}", self.unit, other.unit);
        self.val /= other.val;
        self.unit /= other.unit;
    }
}

impl Div<f64> for &Scaler {
    type Output = Scaler;

    fn div(self, other: f64) -> Scaler {
        Scaler { val: self.val / other, unit: self.unit.clone() }
    }
}

impl Div<&Scaler> for f64 {
    type Output = Scaler;

    fn div(self, other: &Scaler) -> Scaler {
        Scaler { val: self / other.val, unit: other.unit }
    }
}