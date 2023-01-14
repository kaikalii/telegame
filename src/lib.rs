mod draw;
mod input;
mod server;
mod utils;
mod websocket;

pub use glam;
pub use {draw::*, input::*, server::*};

pub trait Game: Send + 'static {
    type State: Send;
    fn new_state(&mut self) -> Self::State;
    fn make_frame(state: &mut Self::State, input: Input) -> Frame;
}
