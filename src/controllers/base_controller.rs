/**
* Copyright (c) AWildDevAppears
*/

pub struct BaseController {}

impl BaseController {
    pub fn new() -> Self {
        Self {}
    }

    fn preload() {}
    fn update() {}
    fn draw() {}
}

pub fn init() -> BaseController {
    BaseController::new()
}
