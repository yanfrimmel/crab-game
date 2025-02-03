use crate::helpers::triangle_rectangle_intersection;
use crate::objects::Block;
use crate::objects::Crab;
use macroquad::prelude::Vec2;
use macroquad::prelude::{is_key_down, KeyCode};
use std::f32::consts::PI;

const SPEED: f32 = 7000.0;

pub fn player_movement(crab: &mut Crab, block: &Block, delta: f32) {
    let distance = delta * SPEED;

    // Get the Block's bounds
    let block_bounds = (block.x, block.y, block.w, block.h);
    // Rotate left leg with A and D keys
    if is_key_down(KeyCode::A) {
        let success = rotate_around(
            &mut crab.left_leg.0,
            &mut crab.left_leg.2,
            crab.left_leg.1,
            crab.leg_rotate_speed,
            -delta,
            block,
        );
        if !success {
            println!("Left leg rotation blocked by collision!");
        }
    }
    if is_key_down(KeyCode::D) {
        let success = rotate_around(
            &mut crab.left_leg.0,
            &mut crab.left_leg.2,
            crab.left_leg.1,
            crab.leg_rotate_speed,
            delta,
            block,
        );
        if !success {
            println!("Left leg rotation blocked by collision!");
        }
    }

    // Rotate right leg with Left and Right arrow keys
    if is_key_down(KeyCode::Left) {
        let success = rotate_around(
            &mut crab.right_leg.0,
            &mut crab.right_leg.2,
            crab.right_leg.1,
            crab.leg_rotate_speed,
            -delta,
            block,
        );
        if !success {
            println!("Right leg rotation blocked by collision!");
        }
    }
    if is_key_down(KeyCode::Right) {
        let success = rotate_around(
            &mut crab.right_leg.0,
            &mut crab.right_leg.2,
            crab.right_leg.1,
            crab.leg_rotate_speed,
            delta,
            block,
        );
        if !success {
            println!("Right leg rotation blocked by collision!");
        }
    }

    // Jump
    if is_key_down(KeyCode::Space) && !crab.falling {
        let new_torso = (
            Vec2::new(crab.torso.0.x, crab.torso.0.y - distance),
            Vec2::new(crab.torso.1.x, crab.torso.1.y - distance),
            Vec2::new(crab.torso.2.x, crab.torso.2.y - distance),
        );

        if !triangle_rectangle_intersection(new_torso, block_bounds) {
            crab.update_position_y(-distance);
        } else {
            println!("Torso movement blocked by collision!");
        }
    }
}

pub fn rotate_around(
    vec0: &mut Vec2,
    vec1: &mut Vec2,
    anchor: Vec2,
    degrees: f32,
    delta: f32,
    block: &Block,
) -> bool {
    let angle: f32 = degrees * PI / 180.0 * delta;
    let diff0 = *vec0 - anchor;
    let diff1 = *vec1 - anchor;

    // Calculate new positions after rotation
    let new_vec0_x = diff0.x * angle.cos() - diff0.y * angle.sin() + anchor.x;
    let new_vec0_y = diff0.y * angle.cos() + diff0.x * angle.sin() + anchor.y;
    let new_vec1_x = diff1.x * angle.cos() - diff1.y * angle.sin() + anchor.x;
    let new_vec1_y = diff1.y * angle.cos() + diff1.x * angle.sin() + anchor.y;

    // Create a new triangle with the rotated points
    let new_tri = (
        Vec2::new(new_vec0_x, new_vec0_y),
        Vec2::new(new_vec1_x, new_vec1_y),
        anchor,
    );

    // Check if the new triangle collides with the Block
    let rect = (block.x, block.y, block.w, block.h);
    if !triangle_rectangle_intersection(new_tri, rect) {
        // No collision, apply the rotation
        vec0.x = new_vec0_x;
        vec0.y = new_vec0_y;
        vec1.x = new_vec1_x;
        vec1.y = new_vec1_y;
        true // Rotation successful
    } else {
        false // Collision detected, rotation blocked
    }
}
