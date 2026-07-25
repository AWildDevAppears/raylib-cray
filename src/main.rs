use raylib::{drawing::RaylibDraw, ffi::Color};

use crate::{gamestate::GameState, views::view_settings_menu::ViewSettingsMenu};

/**
* Copyright (c) AWildDevAppears
*/
mod controllers;
mod gamestate;
mod handlers;
mod managers;
mod views;

fn main() {
    let mut state = GameState::new();
    let (mut game, thread) = raylib::init()
        .size(state.screen_width, state.screen_height)
        .title(state.game_name.as_str())
        .build();

    let scene = ViewSettingsMenu::new(&mut game, &thread);

    while !game.window_should_close() {
        let mut d = game.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        scene.draw(&mut d, &mut state);
    }
}
