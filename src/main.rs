use macroquad::prelude::*;
use player::*;
use text::*;

mod player;
mod text;

#[macroquad::main("RPG")]
async fn main() {
    game().await;
}

async fn game() {
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);

    loop {
        clear_background(WHITE);

        player.update();
        player.draw();

        let (player_x, player_y, player_can_dash) = player.fetch_data();

        draw_all_text(player_x, player_y, player_can_dash);

        next_frame().await;
    }
}
