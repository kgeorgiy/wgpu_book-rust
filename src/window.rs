use std::ops::DerefMut;

use winit::{
    event::{Event, VirtualKeyCode::Escape, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::{Content, WindowConfiguration};
use crate::window_api::RawWindow;

pub fn show<F>(config: &WindowConfiguration, factory: F) -> ! where
    F: FnOnce(&dyn RawWindow) -> Box<dyn Content>,
{
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Create window");
    window.set_title(config.title);

    let mut contents = factory(&window);

    contents
        .deref_mut()
        .resize(window.inner_size().width, window.inner_size().height);

    let render_start_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: window_event, .. } => match window_event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) =>
                    contents.deref_mut().resize(size.width, size.height),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>
                    contents.deref_mut().resize(new_inner_size.width, new_inner_size.height),
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.virtual_keycode == Some(Escape) {
                        *control_flow = ControlFlow::Exit
                    } else {
                        println!("Key: {:?}", input.virtual_keycode);
                    }
                }
                _ => (),
            },
            Event::DeviceEvent { event, .. } => contents.deref_mut().input(event),
            Event::RedrawRequested(_) => contents.deref_mut().update(render_start_time.elapsed()),
            Event::MainEventsCleared => window.request_redraw(),
            _ => (),
        }
    });
}
