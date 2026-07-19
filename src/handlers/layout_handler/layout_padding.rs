/**
* Copyright (c) AWildDevAppears
*/

#[derive(Clone, Copy, Debug, Default)]
pub struct UIElementPadding {
    pub top: f32,
    pub end: f32,
    pub bottom: f32,
    pub start: f32,
}

pub trait IntoPadding {
    fn into_padding(self) -> UIElementPadding;
}

impl IntoPadding for f32 {
    fn into_padding(self) -> UIElementPadding {
        UIElementPadding {
            top: self,
            end: self,
            bottom: self,
            start: self,
        }
    }
}

impl IntoPadding for (f32, f32) {
    fn into_padding(self) -> UIElementPadding {
        let (vertical, horizontal) = self;
        UIElementPadding {
            top: vertical,
            end: horizontal,
            bottom: vertical,
            start: horizontal,
        }
    }
}

impl IntoPadding for (f32, f32, f32) {
    fn into_padding(self) -> UIElementPadding {
        let (top, horizontal, bottom) = self;
        UIElementPadding {
            top,
            end: horizontal,
            bottom,
            start: horizontal,
        }
    }
}

impl IntoPadding for (f32, f32, f32, f32) {
    fn into_padding(self) -> UIElementPadding {
        let (top, right, bottom, left) = self;
        UIElementPadding {
            top,
            end: right,
            bottom,
            start: left,
        }
    }
}
