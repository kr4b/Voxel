use super::constants;

pub fn create_window(event_loop: &winit::event_loop::EventLoop<()>) -> winit::window::Window {
    winit::window::WindowBuilder::new()
        .with_title(constants::WINDOW_TITLE)
        .with_inner_size(winit::dpi::LogicalSize::new(
            constants::WINDOW_WIDTH,
            constants::WINDOW_HEIGHT,
        ))
        .build(event_loop)
        .expect("Failed to create window")
}