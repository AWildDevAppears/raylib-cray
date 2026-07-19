/**
* Copyright (c) AWildDevAppears
*/
use raylib::{ffi::Rectangle, prelude::*};
use std::ffi::CString;

pub use crate::handlers::element::{
    UIElement, UIElementDirection, UIElementSizing, UIElementSizingAxis,
};

fn measure_text_width(text: &str, font_size: i32) -> f32 {
    let c_text = CString::new(text).unwrap_or_else(|_| CString::new("").unwrap());
    unsafe {
        raylib::ffi::MeasureText(c_text.as_ptr(), font_size) as f32
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedElement {
    pub rect: Rectangle,
    pub background: Option<Color>,
    pub text: Option<String>,
    pub text_color: Option<Color>,
    pub font_size: f32,
    pub text_pos: Vector2,
}

pub struct LayoutHandler {
    pub elements: Vec<ResolvedElement>,
    parent_stack: Vec<usize>,
}

impl LayoutHandler {
    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(1024),
            parent_stack: Vec::with_capacity(32),
        }
    }

    pub fn render(
        &mut self,
        element: &UIElement,
        d: &mut impl RaylibDraw,
        x: f32,
        y: f32,
        max_width: f32,
        max_height: f32,
    ) {
        self.elements.clear();
        self.parent_stack.clear();

        // Pass 1: Resolve root dimensions
        let (w, h) = Self::measure_element(element, max_width, max_height);

        // Pass 2: Calculate layouts recursively (Pure CPU calculation)
        self.calculate_layout(element, x, y, w, h);

        // Pass 4: Deferred Drawing Pass
        for el in &self.elements {
            if let Some(color) = el.background {
                d.draw_rectangle_rec(el.rect, color);
            }
            if let Some(ref txt) = el.text {
                let color = el.text_color.unwrap_or(Color::WHITE);
                d.draw_text(txt, el.text_pos.x as i32, el.text_pos.y as i32, el.font_size as i32, color);
            }
        }
    }

    fn close_element(&mut self) {
        self.parent_stack.pop();
    }

    fn measure_element(
        element: &UIElement,
        available_width: f32,
        available_height: f32,
    ) -> (f32, f32) {
        let mut width = match element.sizing.width {
            UIElementSizing::Grow | UIElementSizing::Fit => None,
            width_sizing => Some(width_sizing.resolve(available_width)),
        };

        let mut height = match element.sizing.height {
            UIElementSizing::Grow | UIElementSizing::Fit => None,
            height_sizing => Some(height_sizing.resolve(available_height)),
        };

        // If either width or height is Fit, we must recursively measure children or fit to text.
        if width.is_none() || height.is_none() {
            let padding_vert = element.padding.top + element.padding.bottom;
            let padding_hor = element.padding.start + element.padding.end;
            let inner_w = (available_width - padding_hor).max(0.0);
            let inner_h = (available_height - padding_vert).max(0.0);

            if let Some(ref text) = element.text {
                let text_w = measure_text_width(text, element.style.font_size as i32);
                let text_h = element.style.font_size;

                if width.is_none() {
                    width = Some(text_w + padding_hor);
                }
                if height.is_none() {
                    height = Some(text_h + padding_vert);
                }
            } else if element.children.is_empty() {
                if width.is_none() {
                    width = Some(padding_hor);
                }
                if height.is_none() {
                    height = Some(padding_vert);
                }
            } else {
                let mut children_sizes = Vec::with_capacity(element.children.len());
                let mut total_fixed_w = 0.0;
                let mut total_fixed_h = 0.0;

                for child in &element.children {
                    let (cw, ch) = Self::measure_element(child, inner_w, inner_h);
                    children_sizes.push((cw, ch));

                    if !matches!(child.sizing.width, UIElementSizing::Grow) {
                        total_fixed_w += cw;
                    }
                    if !matches!(child.sizing.height, UIElementSizing::Grow) {
                        total_fixed_h += ch;
                    }
                }

                let num_gaps = (element.children.len() - 1) as f32;
                let gap_total = num_gaps * element.gap;

                match element.direction {
                    UIElementDirection::Horizontal => {
                        if width.is_none() {
                            width = Some(total_fixed_w + gap_total + padding_hor);
                        }
                        if height.is_none() {
                            let max_child_h = children_sizes
                                .iter()
                                .map(|s| s.1)
                                .fold(0.0f32, |a, b| a.max(b));
                            height = Some(max_child_h + padding_vert);
                        }
                    }
                    UIElementDirection::Vertical => {
                        if width.is_none() {
                            let max_child_w = children_sizes
                                .iter()
                                .map(|s| s.0)
                                .fold(0.0f32, |a, b| a.max(b));
                            width = Some(max_child_w + padding_hor);
                        }
                        if height.is_none() {
                            height = Some(total_fixed_h + gap_total + padding_vert);
                        }
                    }
                }
            }
        }

        // Default Grow elements to fill available space if not arranged yet
        (
            width.unwrap_or_else(|| element.sizing.width.resolve(available_width)),
            height.unwrap_or_else(|| element.sizing.height.resolve(available_height)),
        )
    }

    fn calculate_layout(&mut self, element: &UIElement, x: f32, y: f32, width: f32, height: f32) {
        let mut text_pos = Vector2::new(x + element.padding.start, y + element.padding.top);
        if let Some(ref text) = element.text {
            let text_w = measure_text_width(text, element.style.font_size as i32);
            let text_h = element.style.font_size;
            // Center the text inside the element's bounding box
            let tx = x + element.padding.start + (width - element.padding.start - element.padding.end - text_w).max(0.0) / 2.0;
            let ty = y + element.padding.top + (height - element.padding.top - element.padding.bottom - text_h).max(0.0) / 2.0;
            text_pos = Vector2::new(tx, ty);
        }

        let new_index = self.elements.len();
        self.elements.push(ResolvedElement {
            rect: Rectangle {
                x,
                y,
                width,
                height,
            },
            background: element.style.background,
            text: element.text.clone(),
            text_color: element.style.text_color,
            font_size: element.style.font_size,
            text_pos,
        });

        if element.children.is_empty() {
            return;
        }

        let padding_vert = element.padding.top + element.padding.bottom;
        let padding_hor = element.padding.start + element.padding.end;
        let inner_w = (width - padding_hor).max(0.0);
        let inner_h = (height - padding_vert).max(0.0);

        let num_gaps = (element.children.len() - 1) as f32;
        let gap_total = num_gaps * element.gap;

        let mut children_sizes = Vec::with_capacity(element.children.len());
        for child in &element.children {
            let (cw, ch) = Self::measure_element(child, inner_w, inner_h);
            children_sizes.push((cw, ch));
        }

        // Count how many children want to Grow along the main axis, and sum the size of non-grow children
        let mut grow_count = 0;
        let mut total_fixed_size = 0.0;
        for (i, child) in element.children.iter().enumerate() {
            match element.direction {
                UIElementDirection::Horizontal => {
                    if matches!(child.sizing.width, UIElementSizing::Grow) {
                        grow_count += 1;
                    } else {
                        total_fixed_size += children_sizes[i].0;
                    }
                }
                UIElementDirection::Vertical => {
                    if matches!(child.sizing.height, UIElementSizing::Grow) {
                        grow_count += 1;
                    } else {
                        total_fixed_size += children_sizes[i].1;
                    }
                }
            }
        }

        // Distribute remaining space to Grow siblings
        let main_axis_inner_size = match element.direction {
            UIElementDirection::Horizontal => inner_w,
            UIElementDirection::Vertical => inner_h,
        };
        let remaining_space = (main_axis_inner_size - total_fixed_size - gap_total).max(0.0);
        let grow_size = if grow_count > 0 {
            remaining_space / grow_count as f32
        } else {
            0.0
        };

        // Arrange children sequentially
        let mut child_x = x + element.padding.start;
        let mut child_y = y + element.padding.top;

        self.parent_stack.push(new_index);

        for (i, child) in element.children.iter().enumerate() {
            let mut cw = children_sizes[i].0;
            let mut ch = children_sizes[i].1;

            match element.direction {
                UIElementDirection::Horizontal => {
                    if matches!(child.sizing.width, UIElementSizing::Grow) {
                        cw = grow_size;
                    }
                    if matches!(child.sizing.height, UIElementSizing::Grow) {
                        ch = inner_h;
                    }
                }
                UIElementDirection::Vertical => {
                    if matches!(child.sizing.width, UIElementSizing::Grow) {
                        cw = inner_w;
                    }
                    if matches!(child.sizing.height, UIElementSizing::Grow) {
                        ch = grow_size;
                    }
                }
            }

            self.calculate_layout(child, child_x, child_y, cw, ch);

            match element.direction {
                UIElementDirection::Horizontal => {
                    child_x += cw + element.gap;
                }
                UIElementDirection::Vertical => {
                    child_y += ch + element.gap;
                }
            }
        }

        self.close_element();
    }
}
