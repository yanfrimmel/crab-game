use macroquad::prelude::*;

const SPEED: f32 = 200.0;
const SCALE: f32 = 500.0;

pub struct Crab {
    pub torso: (Vec2, Vec2, Vec2),
}

impl Crab {
    pub const fn new(torso: (Vec2, Vec2, Vec2)) -> Self {
        Self { torso }
    }
}

#[macroquad::main("crab-game")]
async fn main() {
    let mut crab = Crab::new((
        Vec2::new(0.0, 0.1) * SCALE,
        Vec2::new(0.2, 0.1) * SCALE,
        Vec2::new(0.1, 0.2) * SCALE,
    ));

    loop {
        let delta = get_frame_time();
        clear_background(GRAY);
        draw_text(&format!("FPS: {}", 1. / delta), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(
            &format!("x1: {} y2: {}", &crab.torso.1.x, &crab.torso.2.y),
            20.0,
            40.0,
            30.0,
            DARKGRAY,
        );
        draw_crab(&crab);
        player_movement(&mut crab.torso, delta);
        next_frame().await
    }
}

fn draw_crab(crab: &Crab) {
    draw_triangle(crab.torso.0, crab.torso.1, crab.torso.2, RED);
}

fn player_movement(crab: &mut (Vec2, Vec2, Vec2), delta: f32) {
    let velocity = delta * SPEED;
    if is_key_down(KeyCode::Left) && crab.0.x > 0.0 {
        crab.0.x -= velocity;
        crab.1.x -= velocity;
        crab.2.x -= velocity;
    }
    if is_key_down(KeyCode::Right) && crab.1.x < screen_width() {
        crab.0.x += velocity;
        crab.1.x += velocity;
        crab.2.x += velocity;
    }
    if is_key_down(KeyCode::Up) && crab.0.y > 0.0 {
        crab.0.y -= velocity;
        crab.1.y -= velocity;
        crab.2.y -= velocity;
    }
    if is_key_down(KeyCode::Down) && crab.2.y < screen_height() {
        crab.0.y += velocity;
        crab.1.y += velocity;
        crab.2.y += velocity;
    }
}
