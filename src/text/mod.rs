use macroquad::{color::BLACK, text::draw_text};

pub fn draw_all_text(x: f32, y: f32, can_dash: bool) {
    draw_text(&format!("Player X: {:.1}", x), 10.0, 20.0, 20.0, BLACK);
    draw_text(&format!("Player Y: {:.1}", y), 10.0, 40.0, 20.0, BLACK);
    draw_text(&format!("Can Dash: {}", can_dash), 10.0, 60.0, 20.0, BLACK);
}
