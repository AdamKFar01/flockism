use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flockism".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}
