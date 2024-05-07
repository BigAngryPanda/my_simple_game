mod gpu_backend;
mod render;
mod scene;

use libvktypes::{
    winit,
    window
};

fn main() {
    let event_loop = window::eventloop();

    let window = window::create_window(&event_loop).expect("Failed to create window");

    let backend = gpu_backend::GPUBackend::new(&window);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            },
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            },
            winit::event::Event::RedrawRequested(_) => {

            },
            _ => ()
        }

    });
}
