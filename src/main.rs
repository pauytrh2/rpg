use macroquad::prelude::*;
use player::*;

mod player;

#[macroquad::main("RPG")]
async fn main() {
    game().await;
}

async fn game() {
    let mut player = Player::new(400.0, 300.0);

    loop {
        clear_background(WHITE);

        player.update();
        player.draw();

        let (player_x, player_y) = player.get_position();

        let position_text = format!("X: {}, Y: {}", player_x, player_y);

        draw_text(&position_text, 10.0, 20.0, 20.0, BLACK);

        next_frame().await;
    }
}
