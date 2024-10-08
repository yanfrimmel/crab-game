use crate::helpers::rotate_around;
use crate::objects::Crab;
use macroquad::prelude::{is_key_down, screen_height, screen_width, KeyCode};

const SPEED: f32 = 200.0;

pub fn player_movement(crab: &mut Crab, delta: f32) {
    let velocity = delta * SPEED;

    if is_key_down(KeyCode::A) && crab.torso.0.x > 0.0 {
        rotate_around(
            &mut crab.left_leg.0,
            &mut crab.left_leg.2,
            crab.left_leg.1,
            crab.leg_rotate_speed,
            -delta,
        );
    }
    if is_key_down(KeyCode::D) && crab.torso.1.x < screen_width() {
        rotate_around(
            &mut crab.left_leg.0,
            &mut crab.left_leg.2,
            crab.left_leg.1,
            crab.leg_rotate_speed,
            delta,
        );
    }
    if is_key_down(KeyCode::Left) && crab.torso.0.x > 0.0 {
        rotate_around(
            &mut crab.right_leg.0,
            &mut crab.right_leg.2,
            crab.right_leg.1,
            crab.leg_rotate_speed,
            -delta,
        );
    }
    if is_key_down(KeyCode::Right) && crab.torso.1.x < screen_width() {
        rotate_around(
            &mut crab.right_leg.0,
            &mut crab.right_leg.2,
            crab.right_leg.1,
            crab.leg_rotate_speed,
            delta,
        );
    }
    if is_key_down(KeyCode::Up) && crab.torso.0.y > 0.0 {
        crab.set_velocity_y(-velocity);
    }
    if is_key_down(KeyCode::Down) && crab.torso.2.y < screen_height() {
        crab.set_velocity_y(velocity);
    }
}
