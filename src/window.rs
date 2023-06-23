use std::ops::DerefMut;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use winit::event::VirtualKeyCode::Escape;

use crate::{WindowConfiguration, Content};
use crate::window_api::RawWindow;

pub fn show<F>(config: &WindowConfiguration, factory: F) -> !
    where F: FnOnce(&dyn RawWindow) -> Box<dyn Content>
{
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Create window");
    window.set_title(config.title);

    let mut contents = factory(&window);

    contents.deref_mut().resize(window.inner_size().width, window.inner_size().height);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } =>
                contents.deref_mut().resize(size.width, size.height),
            // Event::WindowEvent { event: WindowEvent::ScaleFactorChanged { new_inner_size, .. } } =>
            //     contents.deref_mut().resize(new_inner_size.width, new_inner_size.height),
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                if input.virtual_keycode == Some(Escape) {
                    *control_flow = ControlFlow::Exit
                } else {
                    println!("Key: {:?}", input.virtual_keycode);
                }
            },
            Event::RedrawRequested(_) => contents.deref_mut().redraw(),
            _ => (),
        }
    });
}
