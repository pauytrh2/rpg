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

    let mut enemies = vec![
        Enemy::new(100.0, 100.0, 30.0, 30.0, 2.0),
        Enemy::new(300.0, 300.0, 30.0, 30.0, 1.5),
        Enemy::new(500.0, 100.0, 30.0, 30.0, 2.5),
    ];

    loop {
        clear_background(WHITE);

        player.update();
        player.draw();

        draw_all_text(player.x, player.y, player.can_dash);

        for enemy in enemies.iter_mut() {
            enemy.update(&player);
            enemy.draw();
        }

        next_frame().await;
    }
}
