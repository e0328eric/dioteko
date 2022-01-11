use dioteko::prelude::*;
use dioteko::text;
use dioteko::textures::texture::Texture;

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 450;

fn main() -> dioteko::Result<()> {
    let window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "raylib [texture] example -texture rectangle",
    )
    .build()?;

    let texture = Texture::load("image/test.png");

    while !window.should_close() {
        let painter = Painter::begin();

        painter.clear_background(BLANK);

        texture.draw_texture(
            (SCREEN_WIDTH as i32 - texture.width()) / 5,
            (SCREEN_HEIGHT as i32 - texture.height()) / 5,
            WHITE,
        );
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
