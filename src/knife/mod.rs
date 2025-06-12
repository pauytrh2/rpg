use crate::player::Player;
use macroquad::prelude::*;

pub struct Knife {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Knife {
    pub fn new(x: f32, y: f32) -> Self {
        Knife {
            x,
            y,
            width: 5.0,
            height: 20.0,
        }
    }

    pub fn update(&mut self, player: &Player) {
        let (player_x, player_y) = player.get_position();
        self.x = player_x - 10.0;
        self.y = player_y;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, GRAY); // Knife drawn as a small rectangle
    }
}
