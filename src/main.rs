use macroquad::prelude::*;
use macroquad::rand::gen_range;

const NUM_FISH: usize = 100;
const R_SEP: f32 = 24.0;
const R_ALIGN: f32 = 50.0;
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
        let velocities: Vec<Vec2> = fishes.iter().map(|f| f.vel).collect();

        for (i, fish) in fishes.iter_mut().enumerate() {
            let mut sep_steer = Vec2::ZERO;
            let mut sep_count = 0;

            let mut align_sum = Vec2::ZERO;
            let mut align_count = 0;

            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                let diff = fish.pos - positions[j];
                let dist = diff.length();

                if dist > 0.0 && dist < R_SEP {
                    sep_steer += diff / (dist * dist);
                    sep_count += 1;
                }
                if dist > 0.0 && dist < R_ALIGN {
                    align_sum += velocities[j];
                    align_count += 1;
                }
            }

            // Separation: steer away from the average of nearby neighbor positions.
            let mut separation_force = Vec2::ZERO;
            if sep_count > 0 {
                sep_steer /= sep_count as f32;
                if sep_steer.length() > 0.0 {
                    let desired = sep_steer.normalize() * MAX_SPEED;
                    separation_force = desired - fish.vel;
                    if separation_force.length() > MAX_FORCE {
                        separation_force = separation_force.normalize() * MAX_FORCE;
                    }
                }
            }

            // Alignment: steer toward the average velocity of nearby neighbors.
            let mut alignment_force = Vec2::ZERO;
            if align_count > 0 {
                let avg_vel = align_sum / align_count as f32;
                if avg_vel.length() > 0.0 {
                    let desired = avg_vel.normalize() * MAX_SPEED;
                    alignment_force = desired - fish.vel;
                    if alignment_force.length() > MAX_FORCE {
                        alignment_force = alignment_force.normalize() * MAX_FORCE;
                    }
                }
            }

            fish.vel += separation_force + alignment_force;

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
