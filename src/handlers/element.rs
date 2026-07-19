use raylib::ffi::Color;

/**
* Copyright (c) AWildDevAppears
*/
use crate::handlers::padding::{IntoPadding, UIElementPadding};

pub struct UIElement {
    pub sizing: UIElementSizingAxis,
    pub padding: UIElementPadding,
    pub gap: f32,
    pub direction: UIElementDirection,
    pub style: UIElementStyle,
    pub children: Vec<UIElement>,
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

// Sizing
#[derive(Clone, Copy)]
pub enum UIElementSizing {
    Fixed(f32),
    Percentage(f32),
    Fit,
    Grow,
}

impl UIElementSizing {
    fn resolve(self, available: f32) -> f32 {
        match self {
            Self::Fixed(val) => val,
            Self::Percentage(percent) => available * percent,
            Self::Fit => 0.0,
            Self::Grow => available,
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
pub struct UIElementStyle {
    pub background: Option<Color>,
}

pub enum UIElementDirection {
    Horizontal,
    Vertical,
}
