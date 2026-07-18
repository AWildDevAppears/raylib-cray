/**
* Copyright (c) AWildDevAppears
*/
use raylib::prelude::*;

use crate::handlers::layout_handler::UIElementSizing::{Fixed, Percentage};

use super::padding::{IntoPadding, UIElementPadding};

pub struct UIElement {
    sizing: UIElementSizingAxis,
    padding: UIElementPadding,
    gap: f32,
    direction: UIElementDirection,
    style: UIElementStyle,
    children: Vec<UIElement>,
}

impl UIElement {
    pub fn new() -> Self {
        Self {
            style: UIElementStyle { background: None },
            children: Vec::new(),
            sizing: UIElementSizingAxis::fixed(0.0, 0.0),
            padding: UIElementPadding::default(),
            gap: 0.0,
            direction: UIElementDirection::Horizontal,
        }
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

    pub fn compute_size(&self, available_x: f32, available_y: f32) -> (f32, f32) {
        let height: f32 = self.sizing.height.resolve(available_y);
        let width: f32 = self.sizing.width.resolve(available_x);

        (width, height)
    }
}

fn open_element(
    element: &UIElement,
    d: &mut impl RaylibDraw,
    x: f32,
    y: f32,
    max_width: f32,
    max_height: f32,
) {
    let (width, height) = element.compute_size(max_width, max_height);

    if let Some(color) = element.style.background {
        d.draw_rectangle_rec(
            Rectangle {
                x,
                y,
                width,
                height,
            },
            color,
        );
    }

    let mut child_x = x + element.padding.start;
    let mut child_y = y + element.padding.top;

    for child in &element.children {
        let padding_vert = element.padding.top + element.padding.bottom;
        let padding_hor = element.padding.start + element.padding.end;

        open_element(
            child,
            d,
            child_x,
            child_y,
            width - padding_hor,
            height - padding_vert,
        );

        let (child_width, child_height) =
            child.compute_size(width - padding_hor, height - padding_vert);

        match element.direction {
            UIElementDirection::Vertical => child_y += child_height + element.gap,
            UIElementDirection::Horizontal => child_x += child_width + element.gap,
        }

        close_element();
    }

    close_element();
}

fn close_element() {}

pub fn render(
    element: &UIElement,
    d: &mut impl RaylibDraw,
    x: f32,
    y: f32,
    max_width: f32,
    max_height: f32,
) {
    open_element(element, d, x, y, max_width, max_height);
}

// Sizing
#[derive(Clone, Copy)]
pub enum UIElementSizing {
    Fixed(f32),
    Percentage(f32),
    Fit,
    Fill,
}

impl UIElementSizing {
    fn resolve(self, available: f32) -> f32 {
        match self {
            Self::Fixed(val) => val,
            Self::Percentage(percent) => available * percent,
            Self::Fit => 0.0,
            Self::Fill => available,
        }
    }
}

pub struct UIElementSizingAxis {
    width: UIElementSizing,
    height: UIElementSizing,
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
struct UIElementStyle {
    background: Option<Color>,
}

enum UIElementDirection {
    Horizontal,
    Vertical,
}
