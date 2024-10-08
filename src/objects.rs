use macroquad::prelude::*;

pub struct Crab {
    pub torso: (Vec2, Vec2, Vec2),
    pub left_leg: (Vec2, Vec2, Vec2),
    pub right_leg: (Vec2, Vec2, Vec2),
    pub leg_rotate_speed: f32,
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
        }
    }

    pub fn set_velocity_x(&mut self, velocity: f32) {
        self.torso.0.x += velocity;
        self.torso.1.x += velocity;
        self.torso.2.x += velocity;

        self.left_leg.0.x += velocity;
        self.left_leg.1.x += velocity;
        self.left_leg.2.x += velocity;

        self.right_leg.0.x += velocity;
        self.right_leg.1.x += velocity;
        self.right_leg.2.x += velocity;
    }

    pub fn set_velocity_y(&mut self, velocity: f32) {
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

    pub fn draw_crab(&mut self) {
        draw_triangle(self.torso.0, self.torso.1, self.torso.2, RED);
        draw_triangle(self.left_leg.0, self.left_leg.1, self.left_leg.2, RED);
        draw_triangle(self.right_leg.0, self.right_leg.1, self.right_leg.2, RED);
    }
}
