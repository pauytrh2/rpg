use macroquad::{
    color::{Color, GRAY},
    shapes::draw_line,
};

use crate::{
    enemy::{self, Enemy},
    utils::lines_intersect,
};

const LENGTH: f32 = 40.0;
const HALF_LENGTH: f32 = LENGTH / 2.0;
const OFFSET_DISTANCE: f32 = 50.0;
const THICKNESS: f32 = 5.0;
const COLOR: Color = GRAY;

pub struct KillBox {
    x: f32,
    y: f32,
    enable: bool,
    angle: f32,
}

impl KillBox {
    pub fn new(x: f32, y: f32) -> Self {
        KillBox {
            x,
            y,
            enable: false,
            angle: 0.0,
        }
    }

    pub fn update(&mut self, x: f32, y: f32, is_dashing: bool, angle: f32) {
        self.x = x;
        self.y = y;
        self.enable = is_dashing;
        self.angle = angle;
    }

    pub fn draw(&self) {
        if self.enable {
            let (x1, y1, x2, y2) = self.calc_position();
            draw_line(x1, y1, x2, y2, THICKNESS, COLOR);
        }
    }

    fn calc_position(&self) -> (f32, f32, f32, f32) {
        let mut center_x = self.x + 25.0;
        let mut center_y = self.y + 25.0;

        center_x += self.angle.cos() * OFFSET_DISTANCE;
        center_y += self.angle.sin() * OFFSET_DISTANCE;

        let angle = self.calc_angle();

        let dx = angle.cos() * HALF_LENGTH;
        let dy = angle.sin() * HALF_LENGTH;

        let x1 = center_x - dx;
        let y1 = center_y - dy;
        let x2 = center_x + dx;
        let y2 = center_y + dy;

        (x1, y1, x2, y2)
    }

    fn calc_angle(&self) -> f32 {
        self.angle + std::f32::consts::FRAC_PI_2
    }

    pub fn collides_with(&self, enemy: &Enemy) -> bool {
        if !self.enable {
            return false;
        }

        let (x1, y1, x2, y2) = self.calc_position();

        let ex = enemy.x;
        let ey = enemy.y;
        let ew = enemy::WIDTH;
        let eh = enemy::HEIGHT;

        let rect_lines = [
            (ex, ey, ex + ew, ey),
            (ex, ey + eh, ex + ew, ey + eh),
            (ex, ey, ex, ey + eh),
            (ex + ew, ey, ex + ew, ey + eh),
        ];

        for &(rx1, ry1, rx2, ry2) in &rect_lines {
            if lines_intersect(x1, y1, x2, y2, rx1, ry1, rx2, ry2) {
                return true;
            }
        }

        false
    }
}
