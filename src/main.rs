use knife::*;
use macroquad::prelude::*;
use player::*;
use text::*;

mod knife;
mod player;
mod text;

#[macroquad::main("RPG")]
async fn main() {
    game().await;
}

async fn game() {
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut knife = Knife::new(0.0, 0.0);

    loop {
        clear_background(WHITE);

        player.update();
        player.draw();

        knife.update(&player);
        knife.draw();

        let (player_x, player_y) = player.get_position();

        draw_all_text(player_x, player_y);

        next_frame().await;
    }
}
