use glam::vec2;
use telegame::*;

fn main() {
    run_server(MyGame);
}

struct MyGame;

impl Game for MyGame {
    type State = ();
    fn new_state(&mut self) -> Self::State {}
    fn make_frame(_state: &mut Self::State, input: Input) -> Frame {
        let mut frame = Frame::default();
        frame.clear();

        frame.color("blue");
        let size = vec2(100.0, 100.0);
        frame.circle(input.window_size / 2.0, 200.0);

        frame.color("red");
        frame.rectangle(input.mouse_pos - size / 2.0, size);

        frame.color("black");
        frame.font("48px Arial");
        frame.text(vec2(100.0, 300.0), "Hello, world!");

        frame
    }
}
