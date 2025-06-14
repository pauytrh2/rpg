pub fn should_spawn(spawn_timer: f32, spawn_interval: f32) -> bool {
    spawn_timer >= spawn_interval
}
