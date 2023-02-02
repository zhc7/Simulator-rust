use std::collections::BTreeMap;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};
pub use BasicUnit::*;

const DIM: usize = 3;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BasicUnit {
    Second,
    Meter,
    Kilogram,
    Ampere,
    Kelvin,
    Mole,
    Candela,
}

#[derive(Clone, Debug, Copy)]
pub struct Unit {
    muls: BTreeMap<BasicUnit, u8>,
    divs: BTreeMap<BasicUnit, u8>,
}

impl Unit {
    pub fn new(unit: Option<BasicUnit>) -> Unit {
        match unit {
            Some(x) => Unit {
                muls: [(x, 1)].iter().cloned().collect(),
                divs: BTreeMap::new(),
            },
            None => Unit {
                muls: BTreeMap::new(),
                divs: BTreeMap::new(),
            }
        }
    }

    fn mul(&mut self, unit: BasicUnit, power: Option<u8>) {
        let power = match power {
            Some(x) => x,
            None => 1
        };
        if self.divs.contains_key(&unit) {
            self.divs.remove(&unit);
        } else {
            self.muls.entry(unit).and_modify(|e| *e += power).or_insert(power);
        }
    }

    fn div(&mut self, unit: BasicUnit, power: Option<u8>) {
        let power = match power {
            Some(x) => x,
            None => 1
        };
        if self.muls.contains_key(&unit) {
            self.muls.remove(&unit);
        } else {
            self.divs.entry(unit).and_modify(|e| *e += power).or_insert(power);
        }
    }
}

impl MulAssign for Unit {
    fn mul_assign(&mut self, other: Unit) {
        for (unit, power) in other.muls.iter() {
            self.mul(*unit, Some(*power));
        }
        for (unit, power) in other.divs.iter() {
            self.divs.entry(*unit).and_modify(|e| *e += power).or_insert(*power);
        }
    }
}

impl DivAssign for Unit {
    fn div_assign(&mut self, other: Unit) {
        for (unit, power) in other.muls.iter() {
            self.divs.entry(*unit).and_modify(|e| *e += power).or_insert(*power);
        }
        for (unit, power) in other.divs.iter() {
            self.muls.entry(*unit).and_modify(|e| *e += power).or_insert(*power);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rvector {
    val: [f64; DIM],
}

impl Rvector {
    fn zero() -> Rvector {
        Rvector { val: [0.0; DIM] }
    }

    fn copy(&self) -> Rvector {
        Rvector { val: self.val }
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
        let mut f = self.copy();
        for i in 0..DIM {
            f.val[i] += other.val[i];
        }
        f
    }
}

impl AddAssign for Rvector {
    fn add_assign(&mut self, other: Rvector) {
        for i in 0..DIM {
            self.val[i] += other.val[i];
        }
    }
}

impl Sub for Rvector {
    type Output = Rvector;

    fn sub(self, other: Rvector) -> Rvector {
        let mut f = self.copy();
        for i in 0..DIM {
            f.val[i] -= other.val[i];
        }
        f
    }
}

impl SubAssign for Rvector {
    fn sub_assign(&mut self, other: Rvector) {
        for i in 0..DIM {
            self.val[i] -= other.val[i];
        }
    }
}

impl Mul<f64> for Rvector {
    type Output = Rvector;

    fn mul(self, other: f64) -> Rvector {
        let mut f = self.copy();
        for i in 0..DIM {
            f.val[i] *= other;
        }
        f
    }
}

impl MulAssign<f64> for Rvector {
    fn mul_assign(&mut self, other: f64) {
        for i in 0..DIM {
            self.val[i] *= other;
        }
    }
}

impl Div<f64> for Rvector {
    type Output = Rvector;

    fn div(self, other: f64) -> Rvector {
        let mut f = self.copy();
        for i in 0..DIM {
            f.val[i] /= other;
        }
        f
    }
}

impl DivAssign<f64> for Rvector {
    fn div_assign(&mut self, other: f64) {
        for i in 0..DIM {
            self.val[i] /= other;
        }
    }
}

pub trait Entity {
    fn get_mass(&self) -> f64;
    fn get_position(&self) -> Rvector;
    fn get_velocity(&self) -> Rvector;
    fn get_force(&self) -> Rvector;
    fn add_force(&mut self, force: Rvector);
    fn tick(&mut self, dt: f64);
}