use macroquad::{color::RED, shapes::draw_rectangle};

use crate::player::Player;

pub struct Healthbar {
    health: f32,
}

impl Healthbar {
    pub fn new(health: f32) -> Self {
        Healthbar { health }
    }

    pub fn update(&mut self, player: &Player) {
        self.health = player.health
    }

    pub fn draw(&self) {
        draw_rectangle(690.0, 580.0, 100.0, 8.0, RED);
    }
}
