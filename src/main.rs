mod controls;
mod helpers;
mod objects;

use crate::controls::player_movement;
use crate::objects::Crab;
use crate::objects::Drawable;
use macroquad::prelude::*;
use objects::Block;

const SCALE: f32 = 1500.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Crab Game".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut crab = Crab::new(SCALE, Vec2::new(screen_width() / 2.0, 0.0));
    let mut block = Block::new(SCALE, Vec2::new(0.0, screen_height() * 3.0 / 4.0));

    loop {
        let delta = get_frame_time();
        clear_background(GRAY);

        draw_text(&format!("FPS: {}", 1. / delta), 20.0, 20.0, 20.0, YELLOW);
        draw_text(
            &format!("Controls: <-, ->, A, D, space."),
            20.0,
            40.0,
            20.0,
            DARKGRAY,
        );

        block.draw();
        crab.draw();

        crab.apply_gravity(delta, &block);
        player_movement(&mut crab, &block, delta);
        next_frame().await
    }
}
