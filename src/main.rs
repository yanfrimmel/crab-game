use macroquad::prelude::*;

const SPEED: f32 = 200.0;
const SCALE: f32 = 500.0;

#[macroquad::main("crab-game")]
async fn main() {
    let mut crab: (Vec2, Vec2, Vec2) = (
        vec2(0.0, 0.1) * SCALE,
        vec2(0.2, 0.1) * SCALE,
        vec2(0.1, 0.2) * SCALE,
    );

    loop {
        let delta = get_frame_time();
        clear_background(WHITE);
        draw_text(&format!("FPS: {}", 1. / delta), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(
            &format!("x1: {} y2: {}", crab.1.x, crab.2.y),
            20.0,
            40.0,
            30.0,
            DARKGRAY,
        );
        draw_triangle(crab.0, crab.1, crab.2, RED);
        player_movement(&mut crab, delta);
        next_frame().await
    }
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
