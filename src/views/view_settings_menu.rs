/**
* Copyright (c) AWildDevAppears
*/
use raylib::{
    RaylibHandle, RaylibThread,
    drawing::RaylibDrawHandle,
    ffi::{Color, MouseButton},
};

use crate::{
    gamestate::GameState,
    handlers::layout_handler::{
        LayoutHandler, MouseEvent, UIElement, UIElementSizing, UIElementSizingAxis,
    },
    managers::font_manager::{FontFormat, FontManager, FontReference},
    router::Route,
};

pub struct ViewSettingsMenu {
    font_manager: FontManager,
    layout_handler: LayoutHandler,
}

impl ViewSettingsMenu {
    pub fn new(game: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            font_manager: FontManager::new(
                game,
                thread,
                &[FontReference {
                    name: "Foo".to_string(),
                    path: "assets/fonts/Roboto-Regular.ttf",
                    format: FontFormat::TTF,
                }],
            ),
            layout_handler: LayoutHandler::new(),
        }
    }

    fn render(&self, state: &GameState) -> UIElement {
        UIElement::new("Wrapper")
            .sizing(UIElementSizingAxis::fixed(
                state.screen_width as f32,
                state.screen_height as f32,
            ))
            .padding(24.0)
            .background(Color::new(40, 44, 52, 255))
            .gap(10.0)
            .children(vec![
                UIElement::new("LeftCol")
                    .sizing(UIElementSizingAxis::composite(
                        UIElementSizing::Percentage(0.5),
                        UIElementSizing::Fixed(80.0),
                    ))
                    .background(Color::new(97, 175, 239, 255))
                    .padding(12.0)
                    .child(
                        UIElement::new("HelloMessage")
                            .sizing(UIElementSizingAxis::composite(
                                UIElementSizing::Fit,
                                UIElementSizing::Fit,
                            ))
                            .text("Hello World!")
                            .text_color(Color::BLACK)
                            .font_size(24.0),
                    ),
                UIElement::new("RightCol")
                    .sizing(UIElementSizingAxis::composite(
                        UIElementSizing::Percentage(0.5),
                        UIElementSizing::Fixed(80.0),
                    ))
                    .background(Color::new(125, 0, 0, 255))
                    .padding(12.0)
                    // .radius(0.5)
                    .child(
                        UIElement::new("CrayMessage")
                            .sizing(UIElementSizingAxis::composite(
                                UIElementSizing::Fit,
                                UIElementSizing::Fit,
                            ))
                            .text("Cray Engine")
                            .text_color(Color::WHITE)
                            .font("Foo".to_string())
                            .font_size(18.0),
                    ),
            ])
    }
}

impl Route for ViewSettingsMenu {
    fn draw(&mut self, draw: &mut RaylibDrawHandle, state: &mut GameState, mouse: &MouseEvent) {
        self.layout_handler.render(
            &self.render(state),
            draw,
            0.0,
            0.0,
            state.screen_width as f32,
            state.screen_height as f32,
            &self.font_manager,
            mouse,
        );
    }
}
