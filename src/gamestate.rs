/**
* Copyright (c) AWildDevAppears
*/

pub struct GameState {
    pub screen_width: i32,
    pub screen_height: i32,
    pub game_name: String,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            screen_width: 640,
            screen_height: 640,
            game_name: "Boilerplate".to_string(),
        }
    }
}
