use dioteko::prelude::*;

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 450;

fn main() -> dioteko::Result<()> {
    let window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        c"raylib [core] example - input mouse",
    )
    .build()?;

    let mut ball_position;
    let mut ball_color = DARKBLUE;

    utility::time::set_target_fps(60);

    while !window.should_close() {
        ball_position = mouse::get_mouse_position();

        if MouseButton::Left.is_pressed() {
            ball_color = MAROON;
        } else if MouseButton::Middle.is_pressed() {
            ball_color = LIME;
        } else if MouseButton::Right.is_pressed() {
            ball_color = DARKBLUE;
        } else if MouseButton::Side.is_pressed() {
            ball_color = PURPLE;
        } else if MouseButton::Extra.is_pressed() {
            ball_color = YELLOW;
        } else if MouseButton::Forward.is_pressed() {
            ball_color = ORANGE;
        } else if MouseButton::Back.is_pressed() {
            ball_color = BEIGE;
        }

        {
            let painter = Painter::new();

            painter.clear_background(RAYWHITE);
            painter.draw_circle_v(ball_position, 50., ball_color);
            painter.draw_text(
                c"move ball with mouse and click mouse button to change color",
                10,
                10,
                20,
                DARKGRAY,
            );
        }
    }

    Ok(())
}
