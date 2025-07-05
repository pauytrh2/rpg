use dash_indicator::*;
use enemy::*;
use healthbar::*;
use killbox::*;
use macroquad::prelude::*;
use parser::*;
use player::*;
use text::*;
use utils::*;

mod dash_indicator;
mod enemy;
mod healthbar;
mod killbox;
mod parser;
mod player;
mod text;
mod utils;

#[macroquad::main("RPG")]
async fn main() {
    game().await;
}

async fn game() {
    let (should_round_fps, is_fullscreen) = parse_args();

    set_fullscreen(is_fullscreen);

    // After calling set_fullscreen() the screen_width() and screen_height() functions still return original (non-fullscreen) dimensions for 3 frames

    // https://github.com/not-fl3/macroquad/issues/237

    // dbg!(format!(
    //     "Width: {}, Height: {}",
    //     screen_width(),
    //     screen_height()
    // ));

    for _ in 0..3 {
        next_frame().await;
    }

    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut killbox = KillBox::new(player.x, player.y);
    let mut dash_indicator = DashIndicator::new(0.0, 0.0);
    let mut healthbar = Healthbar::new(player.health);

    let mut enemies = Vec::new();
    let mut spawn_timer = 0.0;
    const SPAWN_INTERVAL: f32 = 3.0;

    let mut should_draw_text = false;

    loop {
        clear_background(WHITE);

        if is_key_pressed(KeyCode::F3) {
            should_draw_text = !should_draw_text;
        }

        if should_draw_text {
            draw_all_text(player.x, player.y, player.can_dash, should_round_fps);
        }

        spawn_timer += get_frame_time();

        if should_spawn(spawn_timer, SPAWN_INTERVAL) {
            spawn_timer = 0.0;
            spawn_new_enemy(&mut enemies);
        }

        update_enemies(&mut enemies, &killbox, &player);

        killbox.update(player.x, player.y, player.is_dashing, player.dash_angle());
        killbox.draw();

        player.update();
        player.draw();

        dash_indicator.update(&player);
        dash_indicator.draw();

        healthbar.update(&player);
        healthbar.draw();

        next_frame().await;
    }
}
