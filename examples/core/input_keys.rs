use dioteko::prelude::*;

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 450;

fn main() -> dioteko::Result<()> {
    let window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        c"raylib [core] example - input keys",
    )
    .build()?;

    let mut ball_position = Vector2::new(SCREEN_WIDTH as f32 / 2., SCREEN_HEIGHT as f32 / 2.);

    utility::time::set_target_fps(60);

    while !window.should_close() {
        if Key::Right.is_down() {
            ball_position.x += 2.;
        }
        if Key::Left.is_down() {
            ball_position.x -= 2.;
        }
        if Key::Up.is_down() {
            ball_position.y -= 2.;
        }
        if Key::Down.is_down() {
            ball_position.y += 2.;
        }

        {
            let painter = Painter::new();

            painter.clear_background(RAYWHITE);
            painter.draw_text(c"move the ball with arrow keys", 10, 10, 20, DARKGRAY);
            painter.draw_circle_v(ball_position, 50., MAROON);
        }
    }

    Ok(())
}
