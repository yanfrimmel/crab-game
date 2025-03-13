use crate::helpers::{rotate_point, triangle_rectangle_intersection};
use macroquad::prelude::*;

const GRAVITY: f32 = 200.0; // Acceleration due to gravity (pixels per second squared)

pub trait Drawable {
    fn draw(&mut self);
}

pub struct Block {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Block {
    pub fn new(scale: f32, position: Vec2) -> Self {
        Self {
            x: position.x,
            y: position.y,
            h: scale * 0.1,
            w: screen_width(),
        }
    }
}

impl Drawable for Block {
    fn draw(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, LIME);
    }
}

#[derive(Clone, Copy)]
pub struct BodyPart {
    pub p0: Vec2,
    pub p1: Vec2,
    pub p2: Vec2,
    pub touching_ground: bool,
}

impl BodyPart {
    pub fn new(part0: Vec2, part1: Vec2, part2: Vec2) -> Self {
        Self {
            p0: part0,
            p1: part1,
            p2: part2,
            touching_ground: false,
        }
    }

    pub fn tuple(&self) -> (Vec2, Vec2, Vec2) {
        (self.p0, self.p1, self.p2)
    }
}

pub struct Crab {
    pub torso: BodyPart,
    pub left_leg: BodyPart,
    pub right_leg: BodyPart,
    pub leg_rotate_speed: f32,
    pub velocity_y: f32,
    pub falling: bool,
    pub jump_start: f64,
    pub jump_current: f64,
}

impl Drawable for Crab {
    fn draw(&mut self) {
        draw_triangle(self.torso.p0, self.torso.p1, self.torso.p2, RED);
        draw_triangle(self.left_leg.p0, self.left_leg.p1, self.left_leg.p2, RED);
        draw_triangle(self.right_leg.p0, self.right_leg.p1, self.right_leg.p2, RED);
    }
}

impl Crab {
    pub fn new(scale: f32, position: Vec2) -> Self {
        Self {
            torso: BodyPart::new(
                Vec2::new(0.0, 0.0) * scale + position,
                Vec2::new(0.2, 0.0) * scale + position,
                Vec2::new(0.1, 0.1) * scale + position,
            ),
            left_leg: BodyPart::new(
                Vec2::new(0.025, 0.05) * scale + position,
                Vec2::new(0.05, 0.05) * scale + position,
                Vec2::new(0.0375, 0.175) * scale + position,
            ),
            right_leg: BodyPart::new(
                Vec2::new(0.175, 0.05) * scale + position,
                Vec2::new(0.15, 0.05) * scale + position,
                Vec2::new(0.1625, 0.175) * scale + position,
            ),
            leg_rotate_speed: 180.0,
            velocity_y: 0.0, // Initialize vertical velocity to 0
            falling: false,
            jump_start: 0.0,
            jump_current: 0.0,
        }
    }

    pub fn update_position_y(&mut self, velocity: f32) {
        self.torso.p0.y += velocity;
        self.torso.p1.y += velocity;
        self.torso.p2.y += velocity;

        self.left_leg.p0.y += velocity;
        self.left_leg.p1.y += velocity;
        self.left_leg.p2.y += velocity;

        self.right_leg.p0.y += velocity;
        self.right_leg.p1.y += velocity;
        self.right_leg.p2.y += velocity;
    }

    pub fn apply_gravity(&mut self, delta: f32, block: &Block) {
        self.velocity_y += GRAVITY * delta; // Update velocity based on gravity

        // Check if the new position collides with the Block
        let block_bounds = (block.x, block.y, block.w, block.h);

        // Check balance condition
        let contact_points = self.count_contact_points(block);
        if contact_points == 1 {
            self.lose_balance(delta);
            // Handle collision for y
            self.check_block_collision(block_bounds, delta, true);
        } else {
            // Handle collision for y
            self.check_block_collision(block_bounds, delta, false);
        }
    }

    // Make the crab lose balance by tilting
    fn lose_balance(&mut self, delta: f32) {
        if self.falling {
            return;
        }
        // Define the tilt angle (in radians) and the pivot point (center of the torso)
        let tilt_angle = 1.0 * delta; // Adjust the tilt speed

        if self.left_leg.touching_ground {
            self.rotate_crab(tilt_angle, self.left_leg.p2);
            println!("Left leg pivote");
        } else if self.right_leg.touching_ground {
            self.rotate_crab(-tilt_angle, self.right_leg.p2);
            println!("right leg pivote");
        } else if self.torso.touching_ground {
            let mut left = self.torso.p0;
            let mut right = self.torso.p1;
            let mut lowest_pivot = self.torso.p2;

            if self.torso.p0.y > self.torso.p1.y && self.torso.p0.y > self.torso.p2.y {
                lowest_pivot = self.torso.p0;
                left = self.torso.p1;
                right = self.torso.p2;
            } else if self.torso.p1.y > self.torso.p2.y {
                lowest_pivot = self.torso.p1;
                left = self.torso.p2;
                right = self.torso.p0;
            }

            if left.y > right.y {
                self.rotate_crab(-tilt_angle, lowest_pivot);
            } else {
                self.rotate_crab(tilt_angle, lowest_pivot);
            }
            println!("torso pivote");
        } else {
            println!("ERROR: no pivot!");
        }

        // Increase falling speed to make the crab fall faster
        self.velocity_y += GRAVITY * delta * 20.0;
    }

    fn rotate_crab(&mut self, tilt_angle: f32, pivot: Vec2) {
        // Rotate the torso points around the pivot
        self.torso.p0 = rotate_point(self.torso.p0, pivot, tilt_angle);
        self.torso.p1 = rotate_point(self.torso.p1, pivot, tilt_angle);
        self.torso.p2 = rotate_point(self.torso.p2, pivot, tilt_angle);

        // Rotate the left leg points around the pivot
        self.left_leg.p0 = rotate_point(self.left_leg.p0, pivot, tilt_angle);
        self.left_leg.p1 = rotate_point(self.left_leg.p1, pivot, tilt_angle);
        self.left_leg.p2 = rotate_point(self.left_leg.p2, pivot, tilt_angle);

        // Rotate the right leg points around the pivot
        self.right_leg.p0 = rotate_point(self.right_leg.p0, pivot, tilt_angle);
        self.right_leg.p1 = rotate_point(self.right_leg.p1, pivot, tilt_angle);
        self.right_leg.p2 = rotate_point(self.right_leg.p2, pivot, tilt_angle);
    }

    fn check_block_collision(
        &mut self,
        block_bounds: (f32, f32, f32, f32), // Block's bounds (x, y, width, height)
        delta: f32,                         // Delta time for velocity calculation
        rotation: bool,
    ) -> bool {
        if self.check_part_block_collision(self.left_leg, block_bounds, delta, rotation)
            || self.check_part_block_collision(self.right_leg, block_bounds, delta, rotation)
            // Torso should always fix cliping
            || self.check_part_block_collision(self.torso, block_bounds, delta, false)
        {
            true;
        }
        false
    }

    fn check_part_block_collision(
        &mut self,
        new_part: BodyPart,
        block_bounds: (f32, f32, f32, f32),
        delta: f32,
        rotation: bool,
    ) -> bool {
        if triangle_rectangle_intersection(new_part.tuple(), block_bounds) {
            // Collision detected: stop falling and adjust position
            self.velocity_y = 0.0; // Stop falling
            self.falling = false;
            if !rotation {
                let crab_bottom = new_part.p0.y.max(new_part.p1.y).max(new_part.p2.y);
                let block_top = block_bounds.1;
                let offset = block_top - crab_bottom; // Adjust position to sit on top of the Block
                self.update_position_y(offset.floor());
                // TODO: fix tremors
                println!("triangle_rectangle_intersection offset: {}", offset);
            }
            true
        } else {
            if !rotation {
                // No collision: update position based on velocity
                self.update_position_y(self.velocity_y * delta);
            }
            false
        }
    }

    // Count the number of contact points with block
    pub fn count_contact_points(&mut self, block: &Block) -> usize {
        let mut contact_points = 0;

        self.torso.touching_ground = false;
        self.right_leg.touching_ground = false;
        self.left_leg.touching_ground = false;

        // Check torso contact
        let block_bounds = (block.x, block.y, block.w, block.h);
        if triangle_rectangle_intersection(self.torso.tuple(), block_bounds) {
            self.torso.touching_ground = true;
            contact_points += 1;
        }

        // Check left leg contact
        if triangle_rectangle_intersection(self.left_leg.tuple(), block_bounds) {
            contact_points += 1;
            self.left_leg.touching_ground = true;
        }

        // Check right leg contact
        if triangle_rectangle_intersection(self.right_leg.tuple(), block_bounds) {
            contact_points += 1;
            self.right_leg.touching_ground = true;
        }

        contact_points
    }
}
