use glam::{vec2, Vec2};
use telegame::*;

fn main() {
    run_server(MyGame);
}

struct MyGame;

struct State {
    pos: Vec2,
}

impl Game for MyGame {
    type State = State;
    fn new_state(&mut self) -> Self::State {
        State {
            pos: vec2(100.0, 100.0),
        }
    }
    fn make_frame(state: &mut Self::State, input: Input) -> Frame {
        let control = vec2(
            (input.key_down("d") as i32 - input.key_down("a") as i32) as f32,
            (input.key_down("s") as i32 - input.key_down("w") as i32) as f32,
        )
        .normalize_or_zero();
        let dpos = input.dt * 100.0 * control;
        state.pos += dpos;

        let mut frame = Frame::default();
        frame.clear();

        frame.color("blue");
        let size = vec2(100.0, 100.0);
        frame.circle(state.pos, 50.0);

        frame.color("red");
        frame.rectangle(input.mouse_pos - size / 2.0, size);

        frame.color("black");
        frame.font("48px Arial");
        frame.text(vec2(100.0, 300.0), "Hello, world!");

        frame
    }
}
