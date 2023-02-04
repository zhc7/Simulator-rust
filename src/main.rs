#[macro_use]
extern crate impl_ops;

use structures::*;

use crate::entities::Circle;

mod structures;
mod entities;
mod run;
mod consts;


fn main() {
    let a: Circle = Circle::new(Scaler { val: 1.0, unit: METER },
                                Rvector { val: [0.0, 0.0, 0.0], unit: METER },
                                Scaler { val: 1.0, unit: KILOGRAM },
                                Some(Scaler { val: -0.00001, unit: COULOMB }),
                                None,
                                None,
                                None);
    let b: Circle = Circle::new(Scaler { val: 1.0, unit: METER },
                                Rvector { val: [1.0, 0.0, 0.0], unit: METER },
                                Scaler { val: 1.0, unit: KILOGRAM },
                                Some(Scaler { val: 0.0001, unit: COULOMB }),
                                None,
                                None,
                                None);
    let c: Circle = Circle::new(Scaler { val: 1.0, unit: METER },
                                Rvector { val: [-1.0, 0.0, 0.0], unit: METER },
                                Scaler { val: 1.0, unit: KILOGRAM },
                                Some(Scaler { val: 0.0001, unit: COULOMB }),
                                None,
                                None,
                                None);
    let mut entities: Vec<Box<dyn Entity>> = Vec::new();
    entities.push(Box::new(a));
    entities.push(Box::new(b));
    entities.push(Box::new(c));
    run::run(&mut entities, Scaler { val: 1.0, unit: SECOND });
}
