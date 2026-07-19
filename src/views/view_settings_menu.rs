/**
* Copyright (c) AWildDevAppears
*/
use raylib::ffi::{Color, RaylibPalette};

use crate::handlers::layout_handler::{UIElement, UIElementSizing, UIElementSizingAxis};

pub fn view_settings_menu(screen_width: f32, screen_height: f32) -> UIElement {
    UIElement::new()
        .sizing(UIElementSizingAxis::fixed(screen_width, screen_height))
        .padding(24.0)
        .background(Color::new(40, 44, 52, 255))
        .gap(10.0)
        .children(vec![
            UIElement::new()
                .sizing(UIElementSizingAxis::composite(
                    UIElementSizing::Percentage(0.5),
                    UIElementSizing::Fixed(80.0),
                ))
                .background(Color::new(97, 175, 239, 255))
                .padding(12.0)
                .child(
                    UIElement::new()
                        .sizing(UIElementSizingAxis::composite(
                            UIElementSizing::Percentage(1.0),
                            UIElementSizing::Percentage(1.0),
                        ))
                        .background(Color::PINK),
                ),
            UIElement::new()
                .sizing(UIElementSizingAxis::composite(
                    UIElementSizing::Percentage(0.5),
                    UIElementSizing::Fixed(80.0),
                ))
                .background(Color::new(125, 0, 0, 255)),
        ])
}
