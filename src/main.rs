use dioteko::prelude::*;
use dioteko::text;

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 450;

fn main() -> dioteko::Result<()> {
    let window = WindowBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, "dioteko example").build()?;

    while !window.should_close() {
        let painter = Painter::begin();

        painter.clear_background(BLANK);

        text::draw_text(
            "from https://pixabay.com/photos/ink-feather-pen-quill-write-5067880/",
            50,
            20,
            15,
            YELLOW,
        );
    }

    Ok(())
}
