pub fn should_spawn(spawn_timer: f32, spawn_interval: f32) -> bool {
    spawn_timer >= spawn_interval
}

pub fn lines_intersect(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> bool {
    fn ccw(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> bool {
        (cy - ay) * (bx - ax) > (by - ay) * (cx - ax)
    }

    (ccw(x1, y1, x3, y3, x4, y4) != ccw(x2, y2, x3, y3, x4, y4))
        && (ccw(x1, y1, x2, y2, x3, y3) != ccw(x1, y1, x2, y2, x4, y4))
}
