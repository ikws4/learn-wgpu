mod App;

use wgpu::SurfaceError;
use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    futures::executor::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sketch")
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut app = App::App::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window().id() => {
                if !app.input(event) {
                    match event {
                        WindowEvent::Resized(size) => {
                            app.resize(*size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            app.resize(**new_inner_size);
                        }
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: winit::event::ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => {
                            control_flow.set_exit();
                        }
                        _ => (),
                    }
                }
            }
            winit::event::Event::MainEventsCleared => {
                app.window().request_redraw();
            }
            Event::RedrawRequested(window_id) => {
                if window_id == app.window().id() {
                    app.update();
                    match app.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost) => app.resize(app.size),
                        Err(SurfaceError::OutOfMemory) => control_flow.set_exit(),
                        Err(e) => eprintln!("error: {:?}", e),
                    }
                }
            }
            _ => (),
        }
    });
}
