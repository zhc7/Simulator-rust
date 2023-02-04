use crate::structures::{Entity, JOULE, Scaler, SECOND};

const DT: Scaler = Scaler { val: 0.0001, unit: SECOND };
const INTERVAL: f64 = 0.05;

pub fn get_energy(entities: &Vec<Box<dyn Entity>>) -> (Scaler, Scaler, Scaler) {
    let mut potential_energy = Scaler::zero(JOULE);
    let mut kinetic_energy = Scaler::zero(JOULE);
    for i in 0..entities.len() {
        let field = entities[i].get_field();
        for j in 0..entities.len() {
            if i != j {
                // Potential energy will be count twice:a to b and b to a, therefore here divide it by 2
                potential_energy += field.get_potential_energy(&entities[j]) / 2.0;
            }
        }
    }
    for i in 0..entities.len() {
        let entity = &entities[i];
        kinetic_energy += entity.get_kinetic_energy();
    }
    let energy = &potential_energy + &kinetic_energy;
    (potential_energy, kinetic_energy, energy)
}

fn print_status(entities: &Vec<Box<dyn Entity>>) {
    for i in 0..entities.len() {
        let entity = &entities[i];
        println!("Entity {}: {}, {}, {}", i, entity.get_position(), entity.get_velocity(), entity.get_acceleration());
    }
}


pub fn run(entities: &mut Vec<Box<dyn Entity>>, time: Scaler) {
    let mut t = Scaler { val: 0.0, unit: SECOND };

    println!("t = {}", t);
    print_status(entities);
    let (potential_energy, kinetic_energy, energy) = get_energy(entities);
    println!("Potential Energy: {}, Kinetic Energy: {}, Energy: {}",
             potential_energy, kinetic_energy, energy);
    let energy0 = energy;

    while t < time {
        for i in 0..entities.len() {
            let field = entities[i].get_field();
            for j in 0..entities.len() {
                let other = &mut entities[j];
                if i != j {
                    let force = field.get_force(other);
                    other.add_force(force);
                }
            }
        }
        for i in 0..entities.len() {
            let entity = &mut entities[i];
            entity.tick(DT);
        }
        t += DT;

        if (t.val / DT.val) as i32 % (INTERVAL / DT.val) as i32 == 0 {
            println!("t = {}", t);
            print_status(entities);
            let (potential_energy, kinetic_energy, energy) = get_energy(entities);
            println!("Potential Energy: {}, Kinetic Energy: {}, Energy: {}",
                     potential_energy, kinetic_energy, energy);
        }
    }

    println!("t = {}", t);
    print_status(entities);
    let (potential_energy, kinetic_energy, energy) = get_energy(entities);
    println!("Potential Energy: {}, Kinetic Energy: {}, Energy: {}",
             potential_energy, kinetic_energy, energy);
    let delta_energy = energy - &energy0;
    println!("Delta Energy: {}, percentage: {:.4}%", delta_energy, (&delta_energy / energy0).val * 100.0);
}