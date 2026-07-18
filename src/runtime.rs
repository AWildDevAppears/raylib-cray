/**
* Copyright (c) AWildDevAppears
*/
use raylib::prelude::*;

use crate::gamestate::GameState;
use crate::handlers::layout_handler::render;

pub fn setup() -> GameState {
    GameState::new()
}

pub fn preload(_state: &mut GameState) {}

pub fn update(_state: &mut GameState, _game: &RaylibHandle) {}

pub fn draw(state: &mut GameState, game: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = game.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    render(
        &state.current_view,
        &mut d,
        0.0,
        0.0,
        state.screen_width as f32,
        state.screen_height as f32,
    );
}
