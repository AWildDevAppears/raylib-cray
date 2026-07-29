/**
* Copyright (c) AWildDevAppears
*/
use raylib::{
    drawing::RaylibDraw,
    ffi::{Color, MouseButton},
};

use crate::{gamestate::GameState, handlers::layout_handler::MouseEvent, router::Router};

mod gamestate;
mod handlers;
mod managers;
mod methods;
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
        let position = game.get_mouse_position();
        let click_left = game.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        let click_right = game.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT);

        let mouse = MouseEvent {
            position,
            left_pressed: click_left,
            right_pressed: click_right,
        };

        let mut d = game.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if let Some(ref mut route) = router.current {
            route.draw(&mut d, &mut state, &mouse);
        }
    }
}
