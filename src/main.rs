use macroquad::prelude::*;

const SPEED: f32 = 0.2;

#[macroquad::main("crab-game")]
async fn main() {
    let mut crab: (Vec2, Vec2, Vec2) = (vec2(0.0, 0.1), vec2(0.2, 0.1), vec2(0.1, 0.2));

    loop {
        let delta = get_frame_time();
        clear_background(WHITE);

        draw_triangle_scaled(crab.0, crab.1, crab.2, RED, 500.0);

        draw_text(&format!("FPS: {}", 1. / delta), 20.0, 20.0, 30.0, DARKGRAY);
        if is_key_down(KeyCode::Left) {
            crab.0.x -= delta * SPEED;
            crab.1.x -= delta * SPEED;
            crab.2.x -= delta * SPEED;
        }
        if is_key_down(KeyCode::Right) {
            crab.0.x += delta * SPEED;
            crab.1.x += delta * SPEED;
            crab.2.x += delta * SPEED;
        }
        if is_key_down(KeyCode::Up) {
            crab.0.y -= delta * SPEED;
            crab.1.y -= delta * SPEED;
            crab.2.y -= delta * SPEED;
        }
        if is_key_down(KeyCode::Down) {
            crab.0.y += delta * SPEED;
            crab.1.y += delta * SPEED;
            crab.2.y += delta * SPEED;
        }
        next_frame().await
    }
}

fn draw_triangle_scaled(v1: Vec2, v2: Vec2, v3: Vec2, color: Color, scale: f32) {
    draw_triangle(v1 * scale, v2 * scale, v3 * scale, color)
}
