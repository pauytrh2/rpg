use macroquad::{color::BLACK, text::draw_text};

pub fn draw_all_text(x: f32, y: f32, can_dash: bool) {
    draw_text(&format!("Player X: {x:.1}"), 10.0, 20.0, 20.0, BLACK);
    draw_text(&format!("Player Y: {y:.1}"), 10.0, 40.0, 20.0, BLACK);
    draw_text(&format!("Can Dash: {can_dash}"), 10.0, 60.0, 20.0, BLACK);
}
