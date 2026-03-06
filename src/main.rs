/*
By: Tristan St-Gelais
Date: 2026-03-06
Program Details: You can click different buttons to get different images and text
*/

mod modules;

use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Hello world".to_string(),
        window_width: 1400,
        window_height: 700,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut lbl_out = Label::new("Click a button to see some text and images", 100.0, 50.0, 45);
    let mut img_out = StillImage::new("", 400.0, 350.0, 700.0, 50.0, true, 1.0).await;
    let mut btn_school = TextButton::new(100.0, 450.0, 200.0, 50.0, "School", BLACK, GRAY, 40);
    let mut btn_food = TextButton::new(400.0, 450.0, 200.0, 50.0, "Food", BLACK, GRAY, 40);
    let mut btn_color = TextButton::new(700.0, 450.0, 200.0, 50.0, "Color", BLACK, GRAY, 40);
    let btn_exit = TextButton::new(1000.0, 550.0, 200.0, 50.0, "Exit", BLACK, GRAY, 40);

    let texture_manager = TextureManager::new();
    texture_manager
        .preload_with_loading_screen(&["assets/school.png", "assets/food.png", "assets/color.png", "assets/arrow.png"], None)
        .await;
    img_out.set_preload(texture_manager.get_preload("assets/arrow.png").unwrap());

    loop {
        clear_background(WHITE);
        if btn_exit.click() {
            break;
        }
        if btn_school.click() {
            lbl_out.set_text("Bowmanville High School");
            img_out.set_preload(texture_manager.get_preload("assets/school.png").unwrap());
            btn_school.enabled = false;
            btn_color.enabled = true;
            btn_food.enabled = true;
        }
        if btn_food.click() {
            lbl_out.set_text("Sushi");
            img_out.set_preload(texture_manager.get_preload("assets/food.png").unwrap());
            btn_school.enabled = true;
            btn_color.enabled = true;
            btn_food.enabled = false;
        }
        if btn_color.click() {
            lbl_out.set_text("Maroon");
            img_out.set_preload(texture_manager.get_preload("assets/color.png").unwrap());
            btn_school.enabled = true;
            btn_color.enabled = false;
            btn_food.enabled = true;
        }
        lbl_out.draw();
        img_out.draw();

        next_frame().await;
    }
}
