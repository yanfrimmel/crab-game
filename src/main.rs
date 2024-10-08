mod controls;
mod helpers;
mod objects;

use crate::controls::player_movement;
use crate::objects::Crab;
use macroquad::prelude::*;

const SCALE: f32 = 500.0;

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
        crab.draw_crab();
        player_movement(&mut crab, delta);
        next_frame().await
    }
}
