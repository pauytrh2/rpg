use enemy::*;
use macroquad::prelude::*;
use player::*;
use text::*;

mod enemy;
mod player;
mod text;

#[macroquad::main("RPG")]
async fn main() {
    game().await;
}

async fn game() {
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);

    let mut enemies = Vec::new();

    let mut spawn_timer = 0.0;
    let spawn_interval = 3.0;

    loop {
        clear_background(WHITE);

        player.update();
        player.draw();

        draw_all_text(player.x, player.y, player.can_dash);

        spawn_timer += get_frame_time();

        if spawn_timer >= spawn_interval {
            spawn_timer = 0.0;

            let enemy = Enemy::get_new_enemy();
            enemies.push(enemy);
        }

        for enemy in enemies.iter_mut() {
            enemy.update(&player);
            enemy.draw();
        }

        next_frame().await;
    }
}
