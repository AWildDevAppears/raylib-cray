use crate::{handlers::layout_handler::UIElement, views::view_settings_menu::view_settings_menu};

/**
* Copyright (c) AWildDevAppears
*/

pub struct GameState {
    pub screen_width: i32,
    pub screen_height: i32,
    pub game_name: String,
    pub current_view: UIElement,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            screen_width: 640,
            screen_height: 640,
            game_name: "Boilerplate".to_string(),
            current_view: view_settings_menu(640.0, 640.0),
        }
    }
}
