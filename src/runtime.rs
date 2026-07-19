/**
* Copyright (c) AWildDevAppears
*/
use raylib::prelude::*;

use crate::gamestate::GameState;

pub fn setup() -> GameState {
    GameState::new()
}

pub fn preload(_state: &mut GameState) {}

pub fn update(state: &mut GameState, game: &mut RaylibHandle, thread: &RaylibThread) {
    state.font_manager.preload(game, thread, &[]);
}

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
