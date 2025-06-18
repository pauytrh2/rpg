use dash_indicator::*;
use enemy::*;
use killbox::*;
use macroquad::prelude::*;
use player::*;
use text::*;
use utils::*;

mod dash_indicator;
mod enemy;
mod killbox;
mod player;
mod text;
mod utils;

#[macroquad::main("RPG")]
async fn main() {
    game().await;
}

async fn game() {
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut killbox = KillBox::new(player.x, player.y);
    let mut dash_indicator = DashIndicator::new(0.0, 0.0);

    let mut enemies = Vec::new();
    let mut spawn_timer = 0.0;
    const SPAWN_INTERVAL: f32 = 3.0;

    let mut should_draw_text = false;

    loop {
        clear_background(WHITE);

        if is_key_pressed(KeyCode::F3) {
            should_draw_text = !should_draw_text;
        }

        if should_draw_text == true {
            draw_all_text(player.x, player.y, player.can_dash);
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

        next_frame().await;
    }
}
