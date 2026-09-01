use macroquad::prelude::*;
use macroquad::rand::gen_range;

const NUM_FISH: usize = 100;
const R_SEP: f32 = 24.0;
const MAX_FORCE: f32 = 0.1;
const MAX_SPEED: f32 = 3.0;

struct Fish {
    pos: Vec2,
    vel: Vec2,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Flockism".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut fishes: Vec<Fish> = (0..NUM_FISH)
        .map(|_| Fish {
            pos: vec2(gen_range(0.0, screen_width()), gen_range(0.0, screen_height())),
            vel: vec2(gen_range(-2.0, 2.0), gen_range(-2.0, 2.0)),
        })
        .collect();

    loop {
        clear_background(Color::from_rgba(0x11, 0x11, 0x11, 0xFF));

        let positions: Vec<Vec2> = fishes.iter().map(|f| f.pos).collect();

        for (i, fish) in fishes.iter_mut().enumerate() {
            let mut steer = Vec2::ZERO;
            let mut count = 0;

            for (j, &other_pos) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let diff = fish.pos - other_pos;
                let dist = diff.length();
                if dist > 0.0 && dist < R_SEP {
                    steer += diff / (dist * dist);
                    count += 1;
                }
            }

            if count > 0 {
                steer /= count as f32;
                if steer.length() > 0.0 {
                    let desired = steer.normalize() * MAX_SPEED;
                    let mut force = desired - fish.vel;
                    if force.length() > MAX_FORCE {
                        force = force.normalize() * MAX_FORCE;
                    }
                    fish.vel += force;
                }
            }

            if fish.vel.length() > MAX_SPEED {
                fish.vel = fish.vel.normalize() * MAX_SPEED;
            }

            fish.pos += fish.vel;

            if fish.pos.x < 0.0 {
                fish.pos.x += screen_width();
            }
            if fish.pos.x > screen_width() {
                fish.pos.x -= screen_width();
            }
            if fish.pos.y < 0.0 {
                fish.pos.y += screen_height();
            }
            if fish.pos.y > screen_height() {
                fish.pos.y -= screen_height();
            }
        }

        let size = 8.0;
        for fish in &fishes {
            draw_triangle(
                fish.pos + vec2(size, 0.0),
                fish.pos + vec2(-size, size * 0.6),
                fish.pos + vec2(-size, -size * 0.6),
                WHITE,
            );
        }

        next_frame().await;
    }
}
