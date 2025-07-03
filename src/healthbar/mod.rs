use macroquad::{color::RED, shapes::draw_rectangle};

use crate::player::Player;

pub struct Healthbar {
    x: f32,
    y: f32,
}

impl Healthbar {
    pub fn new(x: f32, y: f32) -> Self {
        Healthbar { x, y }
    }

    pub fn update(&mut self, player: &Player) {
        self.x = player.x;
        self.y = player.y;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x + 5.0, self.y - 9.0, 40.0, 4.0, RED);
    }
}
