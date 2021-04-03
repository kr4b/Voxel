use glutin::{
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    Api, Context, ContextBuilder, ContextWrapper, CreationError, GlRequest, PossiblyCurrent,
};

use super::render::Render;

pub struct Window {
    pub width: i32,
    pub height: i32,
    event_loop: EventLoop<()>,
    context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
}

impl Window {
    pub fn new(width: i32, height: i32, title: &str) -> Result<Self, CreationError> {
        let event_loop = EventLoop::new();
        let window_build = WindowBuilder::new()
            .with_resizable(false)
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height));

        let context = ContextBuilder::new()
            .with_vsync(true)
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 3)))
            .build_windowed(window_build, &event_loop)?;

        let context = unsafe { context.make_current().unwrap() };
        context.window().set_cursor_grab(true).unwrap();
        context.window().set_cursor_visible(false);

        Self::load(&context.context(), width, height);

        Ok(Self {
            width,
            height,
            event_loop,
            context,
        })
    }

    pub fn run(self, mut app: Box<dyn Render>) {
        let context = self.context;

        self.event_loop.run(move |event, _, control_flow| {
            let mut key = "";
            let mut delta = (0.0, 0.0);

            *control_flow = ControlFlow::Wait;

            match event {
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::MouseMotion { delta: d } => {
                        delta = d;
                    }
                    _ => (),
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(k) = input.virtual_keycode {
                            match k {
                                VirtualKeyCode::W => key = "W",
                                VirtualKeyCode::S => key = "S",
                                VirtualKeyCode::A => key = "A",
                                VirtualKeyCode::D => key = "D",
                                VirtualKeyCode::G => if input.state == ElementState::Pressed { key = "G" },
                                VirtualKeyCode::Space => key = "SPACE",
                                VirtualKeyCode::LShift | VirtualKeyCode::RShift => key = "SHIFT",
                                VirtualKeyCode::Up => key = "UP",
                                VirtualKeyCode::Down => key = "DOWN",
                                VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                },
                Event::RedrawEventsCleared { .. } => {
                    // let time = std::time::Instant::now();
                    app.render();
                    context.swap_buffers().unwrap();
                    // println!("{:?}", time.elapsed());
                }
                _ => (),
            }

            app.update(key, delta);
        });
    }

    fn load(context: &Context<PossiblyCurrent>, width: i32, height: i32) {
        gl::load_with(|s| context.get_proc_address(s) as *const _);

        unsafe {
            gl::ClearColor(0.1, 0.55, 0.8, 1.0);
            gl::Viewport(0, 0, width, height);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }
}
