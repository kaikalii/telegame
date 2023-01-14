use glam::vec2;
use telegame::*;

fn main() {
    run_server(|input| {
        let mut frame = Frame::default();
        let size = vec2(100.0, 100.0);
        frame.circle(input.window_size / 2.0, 200.0, "blue");
        frame.rectangle(input.mouse_pos - size / 2.0, size, "red");
        frame
    });
}
