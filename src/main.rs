use macroquad::prelude::*;
use macroquad::rand::gen_range;

const NUM_FISH: usize = 100;
const R_SEP: f32 = 24.0;
const R_ALIGN: f32 = 50.0;
const R_COH: f32 = 60.0;
const R_PREDATOR: f32 = 80.0;
const FLEE_MULTIPLIER: f32 = 3.0;
const MAX_FORCE: f32 = 0.1;
const MAX_SPEED: f32 = 3.0;

const SHARK_MAX_FORCE: f32 = 0.05;
const SHARK_MAX_SPEED: f32 = 2.0;
const SHARK_TARGET_RADIUS: f32 = 40.0;

struct Fish {
    pos: Vec2,
    vel: Vec2,
}

struct Shark {
    pos: Vec2,
    vel: Vec2,
    target: Vec2,
}

fn random_point() -> Vec2 {
    vec2(gen_range(0.0, screen_width()), gen_range(0.0, screen_height()))
}

fn draw_boid(pos: Vec2, vel: Vec2, size: f32, color: Color) {
    let angle = vel.y.atan2(vel.x);
    let (sin, cos) = angle.sin_cos();
    let rotate = |x: f32, y: f32| vec2(x * cos - y * sin, x * sin + y * cos);
    draw_triangle(
        pos + rotate(size, 0.0),
        pos + rotate(-size, size * 0.6),
        pos + rotate(-size, -size * 0.6),
        color,
    );
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

    let mut shark = Shark {
        pos: random_point(),
        vel: Vec2::ZERO,
        target: random_point(),
    };

    // Hysteresis for the dominant-rule label: a challenger must beat the
    // current label by this margin to take over, otherwise near-tied totals
    // (Alignment vs. Cohesion in practice) flip the label every frame.
    const DOMINANCE_MARGIN: f32 = 1.2;
    let mut dominant_rule = "Rule 1: Separation";

    loop {
        clear_background(Color::from_rgba(0x11, 0x11, 0x11, 0xFF));

        // Shark wander: steer toward a target point, pick a new random target once close.
        if (shark.target - shark.pos).length() < SHARK_TARGET_RADIUS {
            shark.target = random_point();
        }
        let to_target = shark.target - shark.pos;
        if to_target.length() > 0.0 {
            let desired = to_target.normalize() * SHARK_MAX_SPEED;
            let mut force = desired - shark.vel;
            if force.length() > SHARK_MAX_FORCE {
                force = force.normalize() * SHARK_MAX_FORCE;
            }
            shark.vel += force;
        }
        if shark.vel.length() > SHARK_MAX_SPEED {
            shark.vel = shark.vel.normalize() * SHARK_MAX_SPEED;
        }
        shark.pos += shark.vel;

        if shark.pos.x < 0.0 {
            shark.pos.x += screen_width();
        }
        if shark.pos.x > screen_width() {
            shark.pos.x -= screen_width();
        }
        if shark.pos.y < 0.0 {
            shark.pos.y += screen_height();
        }
        if shark.pos.y > screen_height() {
            shark.pos.y -= screen_height();
        }

        let shark_pos: Vec2 = shark.pos;

        let positions: Vec<Vec2> = fishes.iter().map(|f| f.pos).collect();
        let velocities: Vec<Vec2> = fishes.iter().map(|f| f.vel).collect();

        // Sum of each rule's steering magnitude across the whole flock this frame,
        // used to report which rule is currently driving the flock's behavior most.
        let mut total_separation = 0.0f32;
        let mut total_alignment = 0.0f32;
        let mut total_cohesion = 0.0f32;
        let mut total_flee = 0.0f32;

        for (i, fish) in fishes.iter_mut().enumerate() {
            let mut sep_steer = Vec2::ZERO;
            let mut sep_count = 0;

            let mut align_sum = Vec2::ZERO;
            let mut align_count = 0;

            let mut coh_sum = Vec2::ZERO;
            let mut coh_count = 0;

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
                if dist > 0.0 && dist < R_COH {
                    coh_sum += positions[j];
                    coh_count += 1;
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

            // Cohesion: steer toward the average position (center of mass) of nearby neighbors.
            let mut cohesion_force = Vec2::ZERO;
            if coh_count > 0 {
                let center = coh_sum / coh_count as f32;
                let to_center = center - fish.pos;
                if to_center.length() > 0.0 {
                    let desired = to_center.normalize() * MAX_SPEED;
                    cohesion_force = desired - fish.vel;
                    if cohesion_force.length() > MAX_FORCE {
                        cohesion_force = cohesion_force.normalize() * MAX_FORCE;
                    }
                }
            }

            // Flee: strongly steer away from the shark when within predator range.
            let mut flee_force = Vec2::ZERO;
            let to_shark = fish.pos - shark_pos;
            let shark_dist = to_shark.length();
            if shark_dist > 0.0 && shark_dist < R_PREDATOR {
                let desired = to_shark.normalize() * MAX_SPEED;
                flee_force = desired - fish.vel;
                if flee_force.length() > MAX_FORCE {
                    flee_force = flee_force.normalize() * MAX_FORCE;
                }
                flee_force *= FLEE_MULTIPLIER;
            }

            total_separation += separation_force.length();
            total_alignment += alignment_force.length();
            total_cohesion += cohesion_force.length();
            total_flee += flee_force.length();

            fish.vel += separation_force + alignment_force + cohesion_force + flee_force;

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
            draw_boid(fish.pos, fish.vel, size, WHITE);
        }

        draw_boid(shark.pos, shark.vel, 18.0, RED);

        let totals = [
            (total_separation, "Rule 1: Separation"),
            (total_alignment, "Rule 2: Alignment"),
            (total_cohesion, "Rule 3: Cohesion"),
            (total_flee, "Rule 4: Flee -> Flash Expansion"),
        ];

        let current_value = totals
            .iter()
            .find(|(_, label)| *label == dominant_rule)
            .map(|(value, _)| *value)
            .unwrap_or(0.0);

        let (best_value, best_label) = totals
            .into_iter()
            .max_by(|a, b| a.0.total_cmp(&b.0))
            .unwrap();

        if best_label != dominant_rule && best_value > current_value * DOMINANCE_MARGIN {
            dominant_rule = best_label;
        }

        draw_text(dominant_rule, 12.0, 24.0, 22.0, Color::from_rgba(0xC0, 0xC0, 0xC0, 0xFF));

        next_frame().await;
    }
}
