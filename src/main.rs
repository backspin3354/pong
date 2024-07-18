use pong::State;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::builder().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = State::default();

    event_loop.run_app(&mut state).unwrap();
}
