use macroquad::{
    color::RED,
    input::{KeyCode, is_key_down, is_key_pressed},
    math::Vec2,
    shapes::draw_rectangle,
    time::get_frame_time,
};

pub struct Player {
    x: f32,
    y: f32,
    speed: f32,
    dash_speed: f32,
    dash_time: f32,
    dash_cooldown: f32,
    can_dash: bool,
    is_dashing: bool,
    dash_direction: Vec2,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            speed: 200.0,
            dash_speed: 400.0,
            dash_time: 0.0,
            dash_cooldown: 1.0,
            can_dash: true,
            is_dashing: false,
            dash_direction: Vec2::ZERO,
        }
    }

    pub fn update(&mut self) {
        let mut direction = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }

        direction = direction.normalize_or_zero();

        if self.can_dash && is_key_pressed(KeyCode::Space) {
            self.start_dash(direction);
        }

        if self.is_dashing {
            self.x += self.dash_direction.x * self.dash_speed * get_frame_time();
            self.y += self.dash_direction.y * self.dash_speed * get_frame_time();
            self.dash_time -= get_frame_time();

            if self.dash_time <= 0.0 {
                self.end_dash();
            }
        } else {
            self.x += direction.x * self.speed * get_frame_time();
            self.y += direction.y * self.speed * get_frame_time();
        }

        if !self.can_dash {
            self.dash_cooldown -= get_frame_time();
            if self.dash_cooldown <= 0.0 {
                self.can_dash = true;
                self.dash_cooldown = 1.0;
            }
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 50.0, 50.0, RED);
    }

    fn start_dash(&mut self, direction: Vec2) {
        if self.can_dash {
            self.is_dashing = true;
            self.dash_time = 0.2;
            self.dash_direction = direction;
            self.can_dash = false;
        }
    }

    fn end_dash(&mut self) {
        self.is_dashing = false;
    }

    pub fn fetch_data(&self) -> (f32, f32, bool) {
        (self.x, self.y, self.can_dash)
    }
}
