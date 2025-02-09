mod controls;
mod helpers;
mod objects;

use crate::controls::player_movement;
use crate::objects::Crab;
use crate::objects::Drawable;
use macroquad::prelude::*;
use objects::Block;

const SCALE: f32 = 1500.0;

#[macroquad::main("crab-game")]
async fn main() {
    let mut crab = Crab::new(SCALE, Vec2::new(screen_width() / 2.0, 0.0));
    let mut block = Block::new(
        SCALE,
        Vec2::new(screen_width() / 2.0, screen_height() * 3.0 / 4.0),
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

        block.draw();
        crab.draw();

        crab.apply_gravity(delta, &block);
        player_movement(&mut crab, &block, delta);
        next_frame().await
    }
}
