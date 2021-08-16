mod button_resource;
pub mod keyboard;
pub mod mouse;

pub fn create_window(
    window_title: &str,
    window_width: u32,
    window_height: u32,
    event_loop: &winit::event_loop::EventLoop<()>,
) -> winit::window::Window {
    let window = winit::window::WindowBuilder::new()
        .with_title(window_title)
        .with_inner_size(winit::dpi::LogicalSize::new(window_width, window_height))
        .build(event_loop)
        .expect("Failed to create window");

    window.set_cursor_grab(true).unwrap();
    window.set_cursor_visible(false);

    window
}
