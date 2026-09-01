use macroquad::prelude::*;
use macroquad::rand::gen_range;

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
    let mut fish = Fish {
        pos: vec2(gen_range(0.0, screen_width()), gen_range(0.0, screen_height())),
        vel: vec2(gen_range(-2.0, 2.0), gen_range(-2.0, 2.0)),
    };

    loop {
        clear_background(Color::from_rgba(0x11, 0x11, 0x11, 0xFF));

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

        let size = 8.0;
        draw_triangle(
            fish.pos + vec2(size, 0.0),
            fish.pos + vec2(-size, size * 0.6),
            fish.pos + vec2(-size, -size * 0.6),
            WHITE,
        );

        next_frame().await;
    }
}
