use macroquad::{
    color::{Color, GREEN, ORANGE},
    shapes::{draw_circle, draw_rectangle},
};

use crate::player::Player;

const CIRCLE_COLOR: Color = GREEN;
const RADIUS: f32 = 10.0;

const RECTANGLE_COLOR: Color = ORANGE;
const WIDTH: f32 = 20.0;
const HEIGHT: f32 = 20.0;

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
            true => draw_circle(self.x + 25.0, self.y - 20.0, RADIUS, CIRCLE_COLOR),
            false => draw_rectangle(self.x + 15.0, self.y - 30.0, WIDTH, HEIGHT, RECTANGLE_COLOR),
        }
    }
}
