use macroquad::prelude::*;
fn collide_and_fix(
    x1: f32,
    y1: f32,
    w1: f32,
    h1: f32,
    x2: f32,
    y2: f32,
    w2: f32,
    h2: f32,
    velocity_x: f32,
    velocity_y: f32,
) -> Option<(f32, f32)> {
    if !(x1 + w1 < x2 || x1 > x2 + w2 || y1 + h1 < y2 || y1 > y2 + h2) {
        let overlap_left = (x1 + w1) - x2;
        let overlap_right = (x2 + w2) - x1;
        let overlap_top = (y1 + h1) - y2;
        let overlap_bottom = (y2 + h2) - y1;

        let min_overlap = overlap_left
            .min(overlap_right)
            .min(overlap_top)
            .min(overlap_bottom);

        if min_overlap == overlap_left {
            Some((x1 - overlap_left, y1))
        } else if min_overlap == overlap_right {
            Some((x1 + overlap_right, y1))
        } else if min_overlap == overlap_top {
            Some((x1, y1 - overlap_top))
        } else {
            Some((x1, y1 + overlap_bottom))
        }
    } else {
        None
    }
}
#[macroquad::main("Physic")]
async fn main() {
    let mut x = 100.0;
    let mut y = 100.0;
    let size = 15.0;
    let mut velocity_y = 0.0;
    let mut velocity_x = 0.0;
    let gravity = 980.0;
    let bounce_factor = 0.8;
    let dt = get_frame_time();
    let mut x2 = 80.0;
    let mut y2 = 70.0;
    let r = 20.0;

    loop {
        if is_key_pressed(KeyCode::R) {
            x = 100.0;
            y = 100.0;
        }
        if is_key_down(KeyCode::D) {
            x2 += 200.0 * dt;
        }
        if is_key_down(KeyCode::A) {
            x2 -= 200.0 * dt;
        }
        if is_key_down(KeyCode::W) {
            y2 -= 200.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            y2 += 200.0 * dt;
        }
        velocity_y += gravity * dt;

        y += velocity_y * dt;
        x += velocity_x * dt;

        if y > screen_height() - size {
            y = screen_height() - size;
            velocity_y = -velocity_y * bounce_factor;
        };
        if let Some((new_x, new_y)) =
            collide_and_fix(x, y, size, size, x2, y2, r, r, velocity_x, velocity_y)
        {
            x = new_x;
            y = new_y;
            velocity_x = 0.0;
            velocity_y = -velocity_y * bounce_factor;
            velocity_x = -velocity_x * bounce_factor;
        }

        clear_background(LIGHTGRAY);
        draw_rectangle(x, y, size, size, RED);
        draw_rectangle(x2, y2, r, r, BLUE);
        next_frame().await
    }
}
