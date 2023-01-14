use crate::{Frame, Input};

pub trait Game: Send + 'static {
    type State: Send;
    fn new_state(&mut self) -> Self::State;
    fn make_frame(state: &mut Self::State, input: Input) -> Frame;
}
