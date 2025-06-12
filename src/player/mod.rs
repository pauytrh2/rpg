use macroquad::{
    color::RED,
    input::{KeyCode, is_key_down},
    shapes::draw_rectangle,
    time::get_frame_time,
};

pub struct Player {
    x: f32,
    y: f32,
    speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player { x, y, speed: 300.0 }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.y -= self.speed * get_frame_time();
        }
        if is_key_down(KeyCode::S) {
            self.y += self.speed * get_frame_time();
        }
        if is_key_down(KeyCode::A) {
            self.x -= self.speed * get_frame_time();
        }
        if is_key_down(KeyCode::D) {
            self.x += self.speed * get_frame_time();
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 50.0, 50.0, RED);
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
