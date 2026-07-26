/**
* Copyright (c) AWildDevAppears
*/
use raylib::{
    drawing::RaylibDrawHandle,
    ffi::{Color, Rectangle},
};

use uuid::Uuid;

use crate::{
    gamestate::GameState,
    handlers::layout_handler::layout_padding::{IntoPadding, UIElementPadding},
};

pub struct UIElement {
    pub sizing: UIElementSizingAxis,
    pub padding: UIElementPadding,
    pub gap: f32,
    pub direction: UIElementDirection,
    pub style: UIElementStyle,
    pub children: Vec<UIElement>,
    pub text: Option<String>,
    pub id: String,
    pub render: Option<fn(draw: &mut RaylibDrawHandle, bounds: Rectangle)>,
}

impl UIElement {
    pub fn new() -> Self {
        Self {
            style: UIElementStyle {
                background: None,
                text_color: Some(Color::WHITE),
                font_size: 20.0,
                radius: None,
                border_width: None,
                border_color: None,
                font: None,
            },
            children: Vec::new(),
            sizing: UIElementSizingAxis::fixed(0.0, 0.0),
            padding: UIElementPadding::default(),
            gap: 0.0,
            direction: UIElementDirection::Horizontal,
            text: None,
            id: Uuid::new_v4().to_string(),
            render: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.style.text_color = Some(color);
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.style.font_size = size;
        self
    }

    pub fn child(mut self, child: UIElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<UIElement>) -> Self {
        self.children = children;
        self
    }

    pub fn sizing(mut self, size: UIElementSizingAxis) -> Self {
        self.sizing = size;
        self
    }

    pub fn padding(mut self, padding: impl IntoPadding) -> Self {
        self.padding = padding.into_padding();
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.style.background = Some(color);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn border(mut self, width: f32, color: Color) -> Self {
        self.style.border_width = Some(width);
        self.style.border_color = Some(color);
        self
    }

    pub fn font(mut self, font: String) -> Self {
        self.style.font = Some(font);
        self
    }

    pub fn render(mut self, render: Option<fn(&mut RaylibDrawHandle<'_>, Rectangle)>) -> Self {
        self.render = render;
        self
    }
}
// Sizing
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum UIElementSizing {
    Fixed(f32),
    Percentage(f32),
    Fit,
    Grow,
}

impl UIElementSizing {
    pub fn resolve(self, available: f32) -> f32 {
        match self {
            Self::Fixed(val) => val,
            Self::Percentage(percent) => available * percent,
            Self::Fit => 0.0,
            Self::Grow => available,
        }
    }
}

pub struct UIElementSizingAxis {
    pub width: UIElementSizing,
    pub height: UIElementSizing,
}

impl UIElementSizingAxis {
    pub fn fixed(width: f32, height: f32) -> Self {
        Self {
            width: UIElementSizing::Fixed(width),
            height: UIElementSizing::Fixed(height),
        }
    }

    pub fn percentage(width: f32, height: f32) -> Self {
        Self {
            width: UIElementSizing::Percentage(width),
            height: UIElementSizing::Percentage(height),
        }
    }

    pub fn composite(width: UIElementSizing, height: UIElementSizing) -> Self {
        Self { width, height }
    }
}

// Style
pub struct UIElementStyle {
    pub background: Option<Color>,
    pub text_color: Option<Color>,
    pub font_size: f32,
    pub radius: Option<f32>,
    pub border_width: Option<f32>,
    pub border_color: Option<Color>,
    pub font: Option<String>,
}

#[repr(u8)]
pub enum UIElementDirection {
    Horizontal,
    Vertical,
}
