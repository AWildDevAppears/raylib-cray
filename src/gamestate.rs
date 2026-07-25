/**
* Copyright (c) AWildDevAppears
*/
use crate::handlers::layout_handler::{LayoutHandler, UIElement};

pub struct GameState {
    pub screen_width: i32,
    pub screen_height: i32,
    pub game_name: String,
    pub current_view: UIElement,
    // Handlers
    pub layout_handler: LayoutHandler,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            screen_width: 640,
            screen_height: 640,
            game_name: "Boilerplate".to_string(),
            current_view: UIElement::new(),
            layout_handler: LayoutHandler::new(),
        }
    }
}
