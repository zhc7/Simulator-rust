use crate::structures::{Entity, Rvector};

pub trait Field {
    fn get_force(&self, entity: &impl Entity) -> Rvector;
}