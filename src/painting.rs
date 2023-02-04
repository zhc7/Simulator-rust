use macroquad::prelude::*;

use crate::structures::Entity;

pub async fn paint(entities: &Vec<Box<dyn Entity>>) {
    clear_background(BLUE);
    set_camera(&Camera3D {
        position: vec3(-20., 15., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    });
    draw_grid(20, 1., BLACK, GRAY);
    for entity in entities {
        entity.draw();
    }
    set_default_camera();
    next_frame().await
}
