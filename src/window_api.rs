use core::time::Duration;

use winit::event::DeviceEvent;


// Content

pub trait Content {
    fn resize(&mut self, _width: u32, _height: u32) {}
    fn update(&mut self, _dt: Duration) {}
    fn input(&mut self, _event: &DeviceEvent) {}
}


// NoContent

pub struct NoContent;

impl Content for NoContent {}

// CompositeContent

pub struct CompositeContent {
    parts: Vec<Box<dyn Content>>,
}

impl<const L: usize> From<[Box<dyn Content>; L]> for CompositeContent {
    fn from(parts: [Box<dyn Content>; L]) -> Self {
        CompositeContent {
            parts: parts.into(),
        }
    }
}

impl Content for CompositeContent {
    fn resize(&mut self, width: u32, height: u32) {
        for part in &mut self.parts {
            part.resize(width, height);
        }
    }

    fn update(&mut self, dt: Duration) {
        for part in &mut self.parts {
            part.update(dt);
        }
    }

    fn input(&mut self, event: &DeviceEvent) {
        for part in &mut self.parts {
            part.input(event);
        }
    }
}

// WindowConfiguration

pub struct WindowConfiguration<'a> {
    pub title: &'a str,
}

// RawWindow

pub trait RawWindow: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle
{}

impl<W> RawWindow for W where
    W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle
{}
