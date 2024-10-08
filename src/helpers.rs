use std::f32::consts::PI;

use macroquad::prelude::{draw_text, Vec2, DARKGRAY};

// rotate for degrees per second
pub fn rotate_around(vec0: &mut Vec2, vec1: &mut Vec2, anchor: Vec2, degrees: f32, delta: f32) {
    let angle: f32 = degrees * PI / 180.0 * delta;
    let diff0 = *vec0 - anchor;
    let diff1 = *vec1 - anchor;

    vec0.x = diff0.x * angle.cos() - diff0.y * angle.sin();
    vec0.y = diff0.y * angle.cos() + diff0.x * angle.sin();
    *vec0 += anchor;

    vec1.x = diff1.x * angle.cos() - diff1.y * angle.sin();
    vec1.y = diff1.y * angle.cos() + diff1.x * angle.sin();
    *vec1 += anchor;

    draw_text(&format!("leg0: {}", vec0), 20.0, 60.0, 30.0, DARKGRAY);
}
