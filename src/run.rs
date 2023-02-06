use std::cmp::min;
use std::fmt::Display;
use std::sync::mpsc::Receiver;
use std::thread;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3, Translation3};
use kiss3d::text::Font;
use kiss3d::window::Window;

//use crate::painting::paint;
use crate::structures::{Entity, JOULE, Rvector, Scaler, SECOND};

const DT: Scaler = Scaler { val: 0.0001, unit: SECOND };
const INTERVAL: f64 = 0.05;

const COLORS: [(f32, f32, f32); 8] = [
    (1., 0., 0.),
    (0., 1., 0.),
    (0., 0., 1.),
    (1., 1., 0.),
    (1., 0., 1.),
    (0., 1., 1.),
    (1., 1., 1.),
    (0., 0., 0.)
];

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

fn print_status(entities: &Vec<Box<dyn Entity>>, t: &Scaler) {
    let t = t.val;
    println!("Time: {:.5}", t);
    for i in 0..entities.len() {
        let entity = &entities[i];
        println!("Entity {}: {}, {}, {}", i, entity.get_position(), entity.get_velocity(), entity.get_acceleration());
    }
}

fn get_info(entities: &Vec<Box<dyn Entity>>, t: &Scaler, initial_energy: &Scaler) -> Info {
    let (potential_energy, kinetic_energy, energy) = get_energy(entities);
    let delta_energy = ( &energy - initial_energy ) / initial_energy;
    Info {
        time: t.val,
        initial_energy: initial_energy.val,
        kinetic_energy: kinetic_energy.val,
        potential_energy: potential_energy.val,
        delta_energy: delta_energy.val,
    }
}

struct Info {
    time: f64,
    initial_energy: f64,
    kinetic_energy: f64,
    potential_energy: f64,
    delta_energy: f64,
}

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Time: {:.4} s", self.time)?;
        writeln!(f, "E_0: {:.4} J", self.initial_energy)?;
        writeln!(f, "E_k: {:.4} J", self.kinetic_energy)?;
        writeln!(f, "E_p: {:.4} J", self.potential_energy)?;
        writeln!(f, "delta_E: {:.4}%", self.delta_energy)?;
        Ok(())
    }
}

fn painter(info_receiver: Receiver<Info>, pos_receiver: Receiver<Rvector>, entities: &Vec<Box<dyn Entity>>) -> thread::JoinHandle<()> {
    let mut fns = Vec::new();
    for e in entities {
        fns.push(e.get_draw());
    }
    thread::spawn(move || {
        let mut window = Window::new("Demo");
        window.set_light(Light::StickToCamera);
        window.set_background_color(0.5, 0.5, 0.5);

        let eye = Point3::new(20.0, 20.0, 20.0);
        let at = Point3::origin();
        let mut arc_ball = ArcBall::new(eye, at);

        let mut nodes = Vec::new();
        for i in 0..fns.len() {
            let mut c = fns[i](&mut window);
            let color = COLORS[min(i, 7)];
            c.set_color(color.0, color.1, color.2);
            nodes.push(c);
        }
        let mut info = info_receiver.recv().unwrap();
        while window.render_with_camera(&mut arc_ball) {
            window.draw_line(&Point3::new(0., 0., 0.), &Point3::new(100., 0., 0.), &Point3::new(1., 0., 0.));
            window.draw_line(&Point3::new(0., 0., 0.), &Point3::new(0., 100., 0.), &Point3::new(0., 1., 0.));
            window.draw_line(&Point3::new(0., 0., 0.), &Point3::new(0., 0., 100.), &Point3::new(0., 0., 1.));
            let t_result = info_receiver.try_recv();
            if t_result.is_ok() {
                info = t_result.unwrap();
                for i in 0..3 {
                    let p = pos_receiver.recv().unwrap();
                    nodes[i].set_local_translation(Translation3::new(p[0] as f32, p[1] as f32, p[2] as f32));
                }
            }
            window.draw_text(&format!("{}", info), &Point2::new(10., 10.),
                             50., &Font::default(), &Point3::new(1., 1., 1.));
        }
    })
}

pub fn run(entities: &mut Vec<Box<dyn Entity>>, time: Scaler) {
    let (time_sender, time_receiver) = std::sync::mpsc::channel();
    let (pos_sender, pos_receiver) = std::sync::mpsc::channel();
    let handler = painter(time_receiver, pos_receiver, entities);

    let mut t = Scaler { val: 0.0, unit: SECOND };

    print_status(entities, &t);
    let (potential_energy, kinetic_energy, energy) = get_energy(entities);
    println!("Potential Energy: {}, Kinetic Energy: {}, Energy: {}",
             potential_energy, kinetic_energy, energy);
    let initial_energy = energy;

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
            print_status(entities, &t);
            let (potential_energy, kinetic_energy, energy) = get_energy(entities);
            println!("Potential Energy: {}, Kinetic Energy: {}, Energy: {}",
                     potential_energy, kinetic_energy, energy);
            time_sender.send(get_info(entities, &t, &initial_energy)).unwrap();
            for i in 0..entities.len() {
                let e = &entities[i];
                let p = e.get_position();
                pos_sender.send(p).unwrap();
            }
        }
    }

    print_status(entities, &t);
    println!("{}", get_info(entities, &t, &initial_energy));

    handler.join().unwrap();
}