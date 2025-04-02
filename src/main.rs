use helper::{collisions, reset};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

// use macroquad::audio::Sound;
use macroquad::audio;

mod helper;

mod init_objects;
use init_objects::*;

const STEP: f32 = 0.005;

fn window_conf() -> Conf {
    Conf {
        window_title: "PI".to_owned(),
        fullscreen: true,
        window_height: 1080,
        window_width: 1920,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let hit = audio::load_sound("score.wav").await.unwrap();

    let Objects {
        mut left_square,
        mut  right_square,
        floor
    } = init_objects();

    let mut collision_number = 0;
    let window_size = vec2(200.0, 180.0);

    loop {
        clear_background(BLACK);

        //floor
        floor.custom_draw_rect();
        left_square.custom_draw_rect();
        right_square.custom_draw_rect();

        left_square.gravity(&floor, STEP);
        right_square.gravity(&floor, STEP);

        right_square.slide(&floor, STEP, &mut collision_number, &hit);
        left_square.slide(&floor, STEP, &mut collision_number, &hit);

        collisions(&mut left_square, &mut right_square, &mut collision_number, &hit);
        draw_text(&collision_number.to_string(), screen_width()/2.0, screen_height()/4.0, 60.0, WHITE);
        draw_text("click with mouse anywhere to enable sounds(if on web)", screen_width()/4.0, screen_height()/3.0-30.0, 30.0, WHITE);
        draw_text("This program approximates PI up to 3 digits(10000:1 mass ratio)", screen_width()/4.0, screen_height()/3.0, 30.0, WHITE);
        draw_text("Currently doesn't work for higher ratios(the square clips through the edge)", screen_width()/4.0, screen_height()/3.0+60.0, 30.0, WHITE);
        draw_text("inspired by that one 3blue1brown video", screen_width()/4.0, screen_height()/3.0+90.0, 30.0, WHITE);
        draw_text("try different ratios using the buttons", screen_width()/4.0, screen_height()/3.0+30.0, 30.0, WHITE);


        root_ui().window(
            hash!(),
            vec2(
                screen_width() / 2.0 - 3.0*window_size.x ,
                screen_height() / 2.0 - window_size.y,
            ),
            window_size,
            |ui| {
                if ui.button(vec2(40.0, 25.0), "1 digit") {
                    reset(&mut left_square, &mut right_square, &mut collision_number, 1.0);
                }
                if ui.button(vec2(40.0, 75.0), "2 digits") {
                    reset(&mut left_square, &mut right_square, &mut collision_number, 100.0);
                }
                if ui.button(vec2(40.0, 125.0), "3 digits") {
                    reset(&mut left_square, &mut right_square, &mut collision_number, 10000.0);
                }
            },
        );
        if is_key_pressed(KeyCode::Escape){
            break;
        }
        next_frame().await;
    }

}
