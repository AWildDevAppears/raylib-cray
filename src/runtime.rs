/**
* Copyright (c) AWildDevAppears
*/
use raylib::prelude::*;

use crate::{
    gamestate::GameState,
    managers::font_manager::{FontFormat, FontReference},
    views::view_settings_menu::view_settings_menu,
};

pub fn setup() -> GameState {
    GameState::new()
}

pub fn preload(state: &mut GameState, game: &mut RaylibHandle, thread: &RaylibThread) {
    state.font_manager.preload(
        game,
        thread,
        &[FontReference {
            name: "Foo".to_string(),
            path: "assets/fonts/Roboto-Regular.ttf".to_string(),
            format: FontFormat::TTF,
        }],
    );
    state.current_view =
        view_settings_menu(state.screen_width as f32, state.screen_height as f32, state)
}

pub fn update(_state: &mut GameState, _game: &mut RaylibHandle, _thread: &RaylibThread) {}

pub fn draw(state: &mut GameState, game: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = game.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    state.layout_handler.render(
        &state.current_view,
        &mut d,
        0.0,
        0.0,
        state.screen_width as f32,
        state.screen_height as f32,
    );
}
