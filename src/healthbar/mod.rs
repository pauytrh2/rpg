use macroquad::{
    color::{Color, RED},
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

use crate::player::Player;

const WIDTH: f32 = 100.0;
const HEIGHT: f32 = 8.0;
const COLOR: Color = RED;
const PADDING: f32 = 10.0;

pub struct Healthbar {
    health: f32,
}

impl Healthbar {
    pub fn new(health: f32) -> Self {
        Healthbar { health }
    }

    pub fn update(&mut self, player: &Player) {
        self.health = player.health;
    }

    pub fn draw(&self) {
        let x = screen_width() - WIDTH - PADDING;
        let y = screen_height() - HEIGHT - PADDING;
        draw_rectangle(x, y, WIDTH, HEIGHT, COLOR);
    }
}
