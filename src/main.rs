use dioteko::prelude::*;

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 450;

fn main() -> dioteko::Result<()> {
    let icon = image::Image::load_image("image/test.png");

    let window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "raylib [texture] example -texture rectangle",
    )
    .set_window_icon(&icon)
    //.set_exit_key(Key::Q)
    .build()?;

    let mut green_ball_pos = Vector2 {
        x: SCREEN_WIDTH as f32 / 2.0,
        y: SCREEN_HEIGHT as f32 / 2.0,
    };

    time::set_target_fps(60);

    let mut scaler;
    while !window.should_close() {
        let painter = Painter::begin();

        painter.clear_background(BLANK);

        if Key::LeftControl.is_down() && Key::D.is_down() {
            break;
        }

        if Key::LeftShift.is_down() {
            scaler = 5.0;
        } else {
            scaler = 1.0;
        }

        if Key::Up.is_down() {
            green_ball_pos.y -= scaler * 1.0;
        }
        if Key::Down.is_down() {
            green_ball_pos.y += scaler * 1.0;
        }
        if Key::Left.is_down() {
            green_ball_pos.x -= scaler * 1.0;
        }
        if Key::Right.is_down() {
            green_ball_pos.x += scaler * 1.0;
        }

        shapes::draw_circle_v(green_ball_pos, 10.0, GREEN);
    }

    Ok(())
}
