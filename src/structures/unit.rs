use std::fmt::{Display, Formatter};
use std::ops::{BitXor, Div, DivAssign, Mul, MulAssign};

pub use BasicUnit::*;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
    basic_units: [i32; 7],
}

impl BasicUnit {
    fn index(index: usize) -> BasicUnit {
        match index {
            0 => Second,
            1 => Meter,
            2 => Kilogram,
            3 => Ampere,
            4 => Kelvin,
            5 => Mole,
            6 => Candela,
            _ => panic!("Invalid index"),
        }
    }
}

impl Display for BasicUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Second => write!(f, "s"),
            Meter => write!(f, "m"),
            Kilogram => write!(f, "kg"),
            Ampere => write!(f, "A"),
            Kelvin => write!(f, "K"),
            Mole => write!(f, "mol"),
            Candela => write!(f, "cd"),
        }
    }
}

impl Mul for BasicUnit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Unit {
        let unit = Unit::new(Some(self));
        unit * rhs
    }
}

impl Div for BasicUnit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Unit {
        let unit = Unit::new(Some(self));
        unit / rhs
    }
}

impl Mul<Unit> for BasicUnit {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Unit {
        let mut unit = Unit::new(Some(self));
        unit *= rhs;
        unit
    }
}

impl Div<Unit> for BasicUnit {
    type Output = Unit;

    fn div(self, rhs: Unit) -> Unit {
        let mut unit = Unit::new(Some(self));
        unit /= rhs;
        unit
    }
}

impl BitXor<i32> for BasicUnit {
    type Output = Unit;

    fn bitxor(self, rhs: i32) -> Unit {
        let mut unit = Unit::new(None);
        unit.basic_units[self as usize] = rhs;
        unit
    }
}

impl Unit {
    pub fn new(unit: Option<BasicUnit>) -> Unit {
        let mut basic_units = [0; 7];
        if let Some(unit) = unit {
            basic_units[unit as usize] = 1;
        }
        Unit { basic_units }
    }

    fn mul(&mut self, unit: BasicUnit, power: Option<i32>) {
        let power = match power {
            Some(x) => x,
            None => 1
        };
        self.basic_units[unit as usize] += power as i32;
    }

    fn div(&mut self, unit: BasicUnit, power: Option<i32>) {
        let power = match power {
            Some(x) => x,
            None => 1
        };
        self.basic_units[unit as usize] -= power as i32;
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..7 {
            if self.basic_units[i] > 0 {
                write!(f, " {}", BasicUnit::index(i))?;
                if self.basic_units[i] > 1 {
                    write!(f, "^{}", self.basic_units[i])?;
                }
                write!(f, " ")?;
            }
        }
        let mut cnt = 0;
        for i in 0..7 {
            if self.basic_units[i] < 0 {
                cnt += 1;
            }
        }
        if cnt >= 1 {
            write!(f, "/")?;
        }
        if cnt >= 2 {
            write!(f, "(")?;
        }
        for i in 0..7 {
            if self.basic_units[i] < 0 {
                write!(f, " {}^{} ", BasicUnit::index(i), -self.basic_units[i])?;
            }
        }
        if cnt >= 2 {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl Mul<BasicUnit> for Unit {
    type Output = Unit;

    fn mul(self, rhs: BasicUnit) -> Unit {
        let mut unit = self;
        Unit::mul(&mut unit, rhs, None);
        unit
    }
}

impl Div<BasicUnit> for Unit {
    type Output = Unit;

    fn div(self, rhs: BasicUnit) -> Unit {
        let mut unit = self;
        Unit::div(&mut unit, rhs, None);
        unit
    }
}

impl MulAssign for Unit {
    fn mul_assign(&mut self, other: Unit) {
        for i in 0..7 {
            self.basic_units[i] += other.basic_units[i];
        }
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Unit) -> Unit {
        let mut unit = self;
        unit *= rhs;
        unit
    }
}

impl DivAssign for Unit {
    fn div_assign(&mut self, other: Unit) {
        for i in 0..7 {
            self.basic_units[i] -= other.basic_units[i];
        }
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Unit) -> Unit {
        let mut unit = self;
        unit /= rhs;
        unit
    }
}