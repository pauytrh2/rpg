use macroquad::color::GRAY;
use macroquad::shapes::draw_rectangle;

pub struct KillBox {
    pub x: f32,
    pub y: f32,
}

impl KillBox {
    pub fn new(x: f32, y: f32) -> Self {
        KillBox { x, y }
    }

    pub fn update(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 40.0, 5.0, GRAY);
    }
}
