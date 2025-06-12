use macroquad::{color::BLACK, text::draw_text};

pub fn draw_all_text(player_x: f32, player_y: f32) {
    let position_text = format!("X: {}, Y: {}", player_x, player_y);

    draw_text(&position_text, 10.0, 20.0, 20.0, BLACK);
}
