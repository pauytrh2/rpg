use crate::player::Player;
use macroquad::{color::RED, shapes::draw_rectangle};

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    height: f32,
    width: f32,
    speed: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, height: f32, width: f32, speed: f32) -> Self {
        Enemy {
            x,
            y,
            height,
            width,
            speed,
        }
    }

    pub fn update(&mut self, player: &Player) {
        let direction_x = player.x - self.x;
        let direction_y = player.y - self.y;

        let distance = (direction_x.powi(2) + direction_y.powi(2)).sqrt();
        if distance > 0.0 {
            let move_x = self.speed * (direction_x / distance);
            let move_y = self.speed * (direction_y / distance);
            self.x += move_x;
            self.y += move_y;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, RED);
    }
}
