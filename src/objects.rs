use crate::helpers::triangle_rectangle_intersection;
use macroquad::prelude::*;

const GRAVITY: f32 = 200.0; // Acceleration due to gravity (pixels per second squared)

pub trait Drawable {
    fn draw(&mut self);
}

pub struct Crab {
    pub torso: (Vec2, Vec2, Vec2),
    pub left_leg: (Vec2, Vec2, Vec2),
    pub right_leg: (Vec2, Vec2, Vec2),
    pub leg_rotate_speed: f32,
    pub velocity_y: f32,
    pub falling: bool,
    pub jump_start: f64,
    pub jump_current: f64,
}

pub struct Block {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Crab {
    pub fn new(scale: f32, position: Vec2) -> Self {
        Self {
            torso: (
                Vec2::new(0.0, 0.0) * scale + position,
                Vec2::new(0.2, 0.0) * scale + position,
                Vec2::new(0.1, 0.1) * scale + position,
            ),
            left_leg: (
                Vec2::new(0.025, 0.05) * scale + position,
                Vec2::new(0.05, 0.05) * scale + position,
                Vec2::new(0.0375, 0.175) * scale + position,
            ),
            right_leg: (
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
        self.torso.0.y += velocity;
        self.torso.1.y += velocity;
        self.torso.2.y += velocity;

        self.left_leg.0.y += velocity;
        self.left_leg.1.y += velocity;
        self.left_leg.2.y += velocity;

        self.right_leg.0.y += velocity;
        self.right_leg.1.y += velocity;
        self.right_leg.2.y += velocity;
    }

    pub fn apply_gravity(&mut self, delta: f32, block: &Block) {
        self.velocity_y += GRAVITY * delta; // Update velocity based on gravity

        // Calculate the new position of the Crab's torso
        let new_torso = (
            Vec2::new(self.torso.0.x, self.torso.0.y + self.velocity_y * delta),
            Vec2::new(self.torso.1.x, self.torso.1.y + self.velocity_y * delta),
            Vec2::new(self.torso.2.x, self.torso.2.y + self.velocity_y * delta),
        );

        let new_left_leg = (
            Vec2::new(
                self.left_leg.0.x,
                self.left_leg.0.y + self.velocity_y * delta,
            ),
            Vec2::new(
                self.left_leg.1.x,
                self.left_leg.1.y + self.velocity_y * delta,
            ),
            Vec2::new(
                self.left_leg.2.x,
                self.left_leg.2.y + self.velocity_y * delta,
            ),
        );

        let new_right_leg = (
            Vec2::new(
                self.right_leg.0.x,
                self.right_leg.0.y + self.velocity_y * delta,
            ),
            Vec2::new(
                self.right_leg.1.x,
                self.right_leg.1.y + self.velocity_y * delta,
            ),
            Vec2::new(
                self.right_leg.2.x,
                self.right_leg.2.y + self.velocity_y * delta,
            ),
        );

        // Check if the new position collides with the Block
        let block_bounds = (block.x, block.y, block.w, block.h);
        // Handle collision for the left leg
        if self.check_block_collision(new_left_leg, block_bounds, delta) {
            return;
        }
        // Handle collision for the right leg
        if self.check_block_collision(new_right_leg, block_bounds, delta) {
            return;
        }
        // Handle collision for the torso
        if self.check_block_collision(new_torso, block_bounds, delta) {
            return;
        }

        // Check if the Crab has hit the ground
        // Check if the Crab's torso has hit the ground
        let crab_bottom_torso = self.torso.0.y.max(self.torso.1.y).max(self.torso.2.y);
        if self.check_ground_collision(crab_bottom_torso) {
            return;
        }

        // Check if the Crab's left leg has hit the ground
        let crab_bottom_left_leg = self
            .left_leg
            .0
            .y
            .max(self.left_leg.1.y)
            .max(self.left_leg.2.y);
        if self.check_ground_collision(crab_bottom_left_leg) {
            return;
        }

        // Check if the Crab's right leg has hit the ground
        let crab_bottom_right_leg = self
            .right_leg
            .0
            .y
            .max(self.right_leg.1.y)
            .max(self.right_leg.2.y);
        self.check_ground_collision(crab_bottom_right_leg);

        // Check balance condition
        let contact_points = self.count_contact_points(block);
        if contact_points < 2 {
            self.lose_balance(delta);
        }
    }

    // Make the crab lose balance by tilting
    fn lose_balance(&mut self, delta: f32) {
        if self.falling {
            return;
        }
        // Define the tilt angle (in radians) and the pivot point (center of the torso)
        let tilt_angle = 0.5 * delta; // Adjust the tilt speed
        let pivot = Vec2::new(
            (self.torso.0.x + self.torso.1.x + self.torso.2.x) / 3.0, // Center X
            (self.torso.0.y + self.torso.1.y + self.torso.2.y) / 3.0, // Center Y
        );

        // Rotate the torso points around the pivot
        self.torso.0 = self.rotate_point(self.torso.0, pivot, tilt_angle);
        self.torso.1 = self.rotate_point(self.torso.1, pivot, tilt_angle);
        self.torso.2 = self.rotate_point(self.torso.2, pivot, tilt_angle);

        // Rotate the left leg points around the pivot
        self.left_leg.0 = self.rotate_point(self.left_leg.0, pivot, tilt_angle);
        self.left_leg.1 = self.rotate_point(self.left_leg.1, pivot, tilt_angle);
        self.left_leg.2 = self.rotate_point(self.left_leg.2, pivot, tilt_angle);

        // Rotate the right leg points around the pivot
        self.right_leg.0 = self.rotate_point(self.right_leg.0, pivot, tilt_angle);
        self.right_leg.1 = self.rotate_point(self.right_leg.1, pivot, tilt_angle);
        self.right_leg.2 = self.rotate_point(self.right_leg.2, pivot, tilt_angle);

        // Increase falling speed to make the crab fall faster
        self.velocity_y += GRAVITY * delta * 2.0;
    }

    // Helper function to rotate a point around a pivot
    fn rotate_point(&self, point: Vec2, pivot: Vec2, angle: f32) -> Vec2 {
        let sin = angle.sin();
        let cos = angle.cos();

        // Translate point back to origin
        let translated_point = Vec2::new(point.x - pivot.x, point.y - pivot.y);

        // Rotate point
        let rotated_point = Vec2::new(
            translated_point.x * cos - translated_point.y * sin,
            translated_point.x * sin + translated_point.y * cos,
        );

        // Translate point back
        Vec2::new(rotated_point.x + pivot.x, rotated_point.y + pivot.y)
    }

    fn check_block_collision(
        &mut self,
        new_part: (Vec2, Vec2, Vec2), // New position of the part (triangle)
        block_bounds: (f32, f32, f32, f32), // Block's bounds (x, y, width, height)
        delta: f32,                   // Delta time for velocity calculation
    ) -> bool {
        if triangle_rectangle_intersection(new_part, block_bounds) {
            // Collision detected: stop falling and adjust position
            self.velocity_y = 0.0; // Stop falling
            self.falling = false;
            let crab_bottom = new_part.0.y.max(new_part.1.y).max(new_part.2.y);
            let block_top = block_bounds.1;
            let offset = block_top - crab_bottom; // Adjust position to sit on top of the Block
            self.update_position_y(offset);
            true
        } else {
            // No collision: update position based on velocity
            self.update_position_y(self.velocity_y * delta);
            false
        }
    }

    // Check if a part of the Crab has hit the ground and adjust its state
    fn check_ground_collision(&mut self, part_bottom: f32) -> bool {
        let ground_level = screen_height();

        if part_bottom >= ground_level {
            // Collision detected: stop falling and adjust position
            self.velocity_y = 0.0; // Stop falling
            self.falling = false;
            self.update_position_y(ground_level - part_bottom); // Snap to the ground
            true // Collision occurred
        } else {
            false // No collision
        }
    }

    // Count the number of contact points with the ground or block
    pub fn count_contact_points(&self, block: &Block) -> usize {
        let mut contact_points = 0;

        // Check torso contact
        let block_bounds = (block.x, block.y, block.w, block.h);
        if triangle_rectangle_intersection(self.torso, block_bounds) {
            contact_points += 1;
        }

        // Check left leg contact
        if triangle_rectangle_intersection(self.left_leg, block_bounds) {
            contact_points += 1;
        }

        // Check right leg contact
        if triangle_rectangle_intersection(self.right_leg, block_bounds) {
            contact_points += 1;
        }

        // Check ground contact
        let ground_level = screen_height();
        if self.torso.0.y >= ground_level
            || self.torso.1.y >= ground_level
            || self.torso.2.y >= ground_level
            || self.left_leg.0.y >= ground_level
            || self.left_leg.1.y >= ground_level
            || self.left_leg.2.y >= ground_level
            || self.right_leg.0.y >= ground_level
            || self.right_leg.1.y >= ground_level
            || self.right_leg.2.y >= ground_level
        {
            contact_points += 1;
        }

        contact_points
    }
}

impl Drawable for Crab {
    fn draw(&mut self) {
        draw_triangle(self.torso.0, self.torso.1, self.torso.2, RED);
        draw_triangle(self.left_leg.0, self.left_leg.1, self.left_leg.2, RED);
        draw_triangle(self.right_leg.0, self.right_leg.1, self.right_leg.2, RED);
    }
}

impl Block {
    pub fn new(scale: f32, position: Vec2) -> Self {
        Self {
            x: position.x,
            y: position.y,
            h: scale * 0.1,
            w: scale * 0.6,
        }
    }
}

impl Drawable for Block {
    fn draw(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, GREEN);
    }
}
