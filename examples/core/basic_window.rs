use dioteko::prelude::*;

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 450;

fn main() -> dioteko::Result<()> {
    let window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "raylib [core] example - basic window",
    )
    .build()?;

    utility::time::set_target_fps(60);

    while !window.should_close() {
        {
            let painter = Painter::new();

            painter.clear_background(RAYWHITE);

            painter.draw_text(
                "Congrats! You created your first window!",
                190,
                200,
                20,
                LIGHTGRAY,
            );
        }
    }

    Ok(())
}
