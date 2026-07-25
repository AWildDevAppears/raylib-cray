use raylib::{drawing::RaylibDraw, ffi::Color};

use crate::{gamestate::GameState, router::Router};

/**
* Copyright (c) AWildDevAppears
*/
mod gamestate;
mod handlers;
mod managers;
mod router;
mod views;

fn main() {
    let mut state = GameState::new();
    let (mut game, thread) = raylib::init()
        .size(state.screen_width, state.screen_height)
        .title(state.game_name.as_str())
        .build();

    let mut router = Router::new();
    router.set_route("settings", &mut game, &thread);

    while !game.window_should_close() {
        let mut d = game.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if let Some(route) = router.current.as_ref() {
            route.draw(&mut d, &mut state);
        }
    }
}
