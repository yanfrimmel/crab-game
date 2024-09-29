use macroquad::prelude::*;

const SPEED: f32 = 200.0;
const SCALE: f32 = 500.0;

pub struct Crab {
    pub torso: (Vec2, Vec2, Vec2),
    pub left_leg: (Vec2, Vec2, Vec2),
    pub right_leg: (Vec2, Vec2, Vec2),
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
}

#[macroquad::main("crab-game")]
async fn main() {
    let mut crab = Crab::new(
        SCALE,
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
    );

    loop {
        let delta = get_frame_time();
        clear_background(GRAY);
        draw_text(&format!("FPS: {}", 1. / delta), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(
            &format!("x0: {} y0: {}", &crab.torso.0.x, &crab.torso.0.y),
            20.0,
            40.0,
            30.0,
            DARKGRAY,
        );
        draw_crab(&crab);
        player_movement(&mut crab, delta);
        next_frame().await
    }
}

fn draw_crab(crab: &Crab) {
    draw_triangle(crab.torso.0, crab.torso.1, crab.torso.2, RED);
    draw_triangle(crab.left_leg.0, crab.left_leg.1, crab.left_leg.2, RED);
    draw_triangle(crab.right_leg.0, crab.right_leg.1, crab.right_leg.2, RED);
}

fn player_movement(crab: &mut Crab, delta: f32) {
    let velocity = delta * SPEED;
    if is_key_down(KeyCode::Left) && crab.torso.0.x > 0.0 {
        crab.set_velocity_x(-velocity);
    }
    if is_key_down(KeyCode::Right) && crab.torso.1.x < screen_width() {
        crab.set_velocity_x(velocity);
    }
    if is_key_down(KeyCode::Up) && crab.torso.0.y > 0.0 {
        crab.set_velocity_y(-velocity);
    }
    if is_key_down(KeyCode::Down) && crab.torso.2.y < screen_height() {
        crab.set_velocity_y(velocity);
    }
}
