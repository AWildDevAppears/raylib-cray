/**
* Copyright (c) AWildDevAppears
*/
mod controllers;
mod gamestate;
mod handlers;
mod runtime;
mod views;

fn main() {
    let mut state = runtime::setup();

    let (mut game, thread) = raylib::init()
        .size(state.screen_width, state.screen_height)
        .title(state.game_name.as_str())
        .build();

    runtime::preload(&mut state);

    while !game.window_should_close() {
        runtime::update(&mut state, &game);

        runtime::draw(&mut state, &mut game, &thread);
    }
}
