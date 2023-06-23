use webgpu_book::window::show;
use webgpu_book::{NoContent, WindowConfiguration};

fn main() -> ! {
    show(
        &WindowConfiguration {
            title: "rust::WebGPU",
        },
        |_window| Box::new(NoContent),
    );
}
