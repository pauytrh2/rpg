use macroquad::color::GRAY;
use macroquad::shapes::draw_rectangle;

pub struct KillBox {
    x: f32,
    y: f32,
    enable: bool,
}

impl KillBox {
    pub fn new(x: f32, y: f32) -> Self {
        KillBox {
            x,
            y,
            enable: false,
        }
    }

    pub fn update(&mut self, x: f32, y: f32, is_dashing: bool) {
        self.x = x;
        self.y = y;
        self.enable = is_dashing;
    }

    pub fn draw(&self) {
        if self.enable {
            draw_rectangle(self.x, self.y, 40.0, 5.0, GRAY);
        }
    }
}
