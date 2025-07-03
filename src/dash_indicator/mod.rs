use macroquad::{
    color::{GREEN, ORANGE},
    shapes::{draw_circle, draw_rectangle},
};

use crate::player::Player;

pub struct DashIndicator {
    x: f32,
    y: f32,
    enabled: bool,
}

impl DashIndicator {
    pub fn new(x: f32, y: f32) -> Self {
        DashIndicator {
            x,
            y,
            enabled: false,
        }
    }

    pub fn update(&mut self, player: &Player) {
        self.enabled = player.can_dash;
        self.x = player.x;
        self.y = player.y;
    }

    pub fn draw(&self) {
        match self.enabled {
            true => draw_circle(self.x + 25.0, self.y - 25.0, 10.0, GREEN),
            false => draw_rectangle(self.x + 15.0, self.y - 35.0, 20.0, 20.0, ORANGE),
        }
    }
}
