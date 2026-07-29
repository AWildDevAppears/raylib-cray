/**
* Copyright (c) AWildDevAppears
*/
use raylib::{RaylibHandle, RaylibThread, drawing::RaylibDrawHandle};

use crate::{
    gamestate::GameState, handlers::layout_handler::MouseEvent,
    views::view_settings_menu::ViewSettingsMenu,
};

pub trait Route {
    fn draw(&mut self, draw: &mut RaylibDrawHandle, state: &mut GameState, mouse: &MouseEvent);
}

pub struct Router {
    routes: &'static [(
        &'static str,
        fn(&mut RaylibHandle, &RaylibThread) -> Box<dyn Route>,
    )],
    pub current: Option<Box<dyn Route>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: &[("settings", |game, thread| {
                Box::new(ViewSettingsMenu::new(game, thread))
            })],
            current: None,
        }
    }

    pub fn set_route(&mut self, name: &str, game: &mut RaylibHandle, thread: &RaylibThread) {
        let (_, factory) = self
            .routes
            .iter()
            .find(|(key, _)| *key == name)
            .expect("Cannot find route with the provided name");

        self.current = Some(factory(game, thread));
    }
}
