use crate::{killbox::KillBox, player::Player};
use macroquad::{
    color::RED,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};
use rand::Rng;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
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

    pub fn get_new_enemy() -> Self {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let mut rng = rand::rng();

        let spawn_margin = 50.0;

        let (x, y) = match rng.random_range(0..4) {
            0 => (
                -spawn_margin,
                rng.random_range(-spawn_margin..screen_height + spawn_margin),
            ),
            1 => (
                screen_width + spawn_margin,
                rng.random_range(-spawn_margin..screen_height + spawn_margin),
            ),
            2 => (
                rng.random_range(-spawn_margin..screen_width + spawn_margin),
                -spawn_margin,
            ),
            _ => (
                rng.random_range(-spawn_margin..screen_width + spawn_margin),
                screen_height + spawn_margin,
            ),
        };

        let speed = rng.random_range(0.5..2.0);

        Enemy::new(x, y, 30.0, 30.0, speed)
    }
}

pub fn update_enemies(enemies: &mut Vec<Enemy>, killbox: &KillBox, player: &Player) {
    for enemy in enemies.iter_mut() {
        enemy.update(&player);
        enemy.draw();
    }

    enemies.retain(|enemy| {
        if killbox.collides_with(enemy) {
            false
        } else {
            true
        }
    });
}

pub fn spawn_new_enemy(enemies: &mut Vec<Enemy>) {
    let enemy = Enemy::get_new_enemy();
    enemies.push(enemy);
}
