use enemy::*;
use killbox::*;
use macroquad::prelude::*;
use player::*;
use text::*;
use utils::*;

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

    let mut enemies = Vec::new();
    let mut spawn_timer = 0.0;
    const SPAWN_INTERVAL: f32 = 3.0;

    loop {
        clear_background(WHITE);

        player.update();
        player.draw();

        killbox.update(player.x, player.y, player.is_dashing);
        killbox.draw();

        draw_all_text(player.x, player.y, player.can_dash);

        spawn_timer += get_frame_time();

        if should_spawn(spawn_timer, SPAWN_INTERVAL) {
            spawn_timer = 0.0;
            spawn_new_enemy(&mut enemies);
        }

        update_enemies(&mut enemies, &player);

        next_frame().await;
    }
}
