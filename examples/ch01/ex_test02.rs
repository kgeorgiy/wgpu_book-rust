use webgpu_book::{NoContent, WindowConfiguration};
use webgpu_book::window::show;

fn main() -> ! {
    show(
        &WindowConfiguration {
            title: "rust::WebGPU",
        },
        |_window| Box::new(NoContent),
    );
}
