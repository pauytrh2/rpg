use macroquad::{
    color::{Color, RED},
    shapes::draw_rectangle,
};

use crate::player::Player;

const X: f32 = 690.0;
const Y: f32 = 580.0;
const WIDTH: f32 = 100.0;
const HEIGHT: f32 = 8.0;
const COLOR: Color = RED;

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
        draw_rectangle(X, Y, WIDTH, HEIGHT, COLOR);
    }
}
