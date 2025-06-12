use macroquad::{color::BLACK, text::draw_text};

pub fn draw_all_text(player_x: f32, player_y: f32, player_can_dash: bool) {
    let position_text = format!("X: {}, Y: {}", player_x, player_y);
    let dash_text = format!("Can Dash: {}", player_can_dash);

    draw_text(&position_text, 10.0, 20.0, 20.0, BLACK);
    draw_text(&dash_text, 10.0, 40.0, 20.0, BLACK);
}
