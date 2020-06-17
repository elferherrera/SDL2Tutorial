use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;

const WINDOW_NAME: &str = "moving";
const WINDOW_HEIGHT: u32 = 200;
const WINDOW_WIDTH: u32 = 300;
const BOX_SIZE: u32 = 50;
const STEP_SIZE: u32 = 10;

fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let video = ctx.video()?;

    let window = video
        .window(WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let creator = canvas.texture_creator();

    let mut box_target = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, BOX_SIZE, BOX_SIZE)
        .map_err(|e| e.to_string())?;

    // Position tuple that will be updated with the arrows or mouse movement
    let mut position = (0, 0);

    'mainloop: loop {
        // The context event pump contains all the posible events that can
        // happen within the window. These include pressed keys, mouse movement,
        // mouse clicks, quit events, etc.
        for event in ctx.event_pump()?.poll_iter() {
            match event {
                // This match will break the main loop when the Escape key is pressed
                // or when the close icon is pressed in the window
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } | Event::Quit { .. } => {
                    break 'mainloop
                },

                // The next events will decide how to move the object
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    // If the new position is negative then the box
                    // will be drawn at the egde of the window
                    let new_y = position.1 as i32 - STEP_SIZE as i32;

                    position.1 = if new_y > 0 { new_y } else { 0 };
                },

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    // If the new position is larger than the window
                    // size then it will be drawn at the edge of the window
                    let new_y = position.1 as i32 + STEP_SIZE as i32;

                    position.1 = if new_y + BOX_SIZE as i32 > WINDOW_HEIGHT as i32 { 
                        WINDOW_HEIGHT as i32 - BOX_SIZE as i32
                    } else { new_y };
                },

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    // If the new position is negative then the box
                    // will be drawn at the egde of the window
                    let new_x = position.0 as i32 - STEP_SIZE as i32;

                    position.0 = if new_x > 0 { new_x } else { 0 };
                },

                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    // If the new position is larger than the window
                    // size then it will be drawn at the edge of the window
                    let new_x = position.0 as i32 + STEP_SIZE as i32;

                    position.0 = if new_x + BOX_SIZE as i32 > WINDOW_WIDTH  as i32 { 
                        WINDOW_WIDTH as i32 - BOX_SIZE as i32
                    } else { new_x };
                },

                Event::MouseMotion { x: x_val, y: y_val, .. } => {
                    // Using the MouseMotion enum to extract the position values
                    // of the mouse
                    position.0 = x_val;
                    position.1 = y_val;
                }
                _ => {},
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        canvas
            .with_texture_canvas(&mut box_target, |texture| {
                texture.set_draw_color(Color::RGB(100, 100, 100));
                texture.clear();
            })
            .map_err(|e| e.to_string())?;

        canvas
            .copy(
                &box_target, 
                None, 
                Rect::new(
                    position.0,
                    position.1,
                    BOX_SIZE,
                    BOX_SIZE)
                )?;

        canvas.present();
    }

    Ok(())
}
