use core::time::Duration;

use winit::event::DeviceEvent;


//
// Content

pub trait Content<T> {
    fn resize(&mut self, _context: T, _width: u32, _height: u32) {}
    fn update(&mut self, _context: T, _dt: Duration) {}
    fn input(&mut self, _context: T, _event: &DeviceEvent) {}
}

//
// NoContent

pub struct NoContent;

impl<T> Content<T> for NoContent {}

// CompositeContent

pub struct CompositeContent<T> {
    pub(crate) parts: Vec<Box<dyn Content<T>>>,
}

impl<T, const L: usize> From<[Box<dyn Content<T>>; L]> for CompositeContent<T> {
    fn from(parts: [Box<dyn Content<T>>; L]) -> Self {
        CompositeContent {
            parts: parts.into(),
        }
    }
}

impl<T: Clone> Content<T> for CompositeContent<T> {
    fn resize(&mut self, context: T, width: u32, height: u32) {
        for part in &mut self.parts {
            part.resize(context.clone(), width, height);
        }
    }

    fn update(&mut self, context: T, dt: Duration) {
        for part in &mut self.parts {
            part.update(context.clone(), dt);
        }
    }

    fn input(&mut self, context: T, event: &DeviceEvent) {
        for part in &mut self.parts {
            part.input(context.clone(), event);
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
