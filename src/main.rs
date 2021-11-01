use winit::event::Event;
use winit::event::ElementState;
use winit::event::WindowEvent;
use winit::event::KeyboardInput;
use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

mod render;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    match pollster::block_on(render::RenderState::new(&window)) {
        Ok(render_state) => {
            println!(
                "Using device {} ({:?})",
                render_state.adapter.get_info().name,
                render_state.adapter.get_info().device_type);

            event_loop.run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested |
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
                _ => {}
            });
        },
        Err(reason) => {
            println!("Failed to initialize rendering: {}", reason);
            std::process::exit(1);
        }
    }
}
