use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Rect, Point};

const WINDOW_NAME: &str = "boxes";
const WINDOW_HEIGHT: u32 = 200;
const WINDOW_WIDTH: u32 = 300;
const BOX_SIZE: u32 = 40;

fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let video = ctx.video()?;

    let window = video
        .window(WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Mutable canvas that will be used to draw all the textures
    // that are created. In this example a texture is used to draw two boxes
    // in a white canvas.
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    // The texture creator can be used to define multiple
    // renderers to create different targets which can be used to create shapes
    // and textures
    let creator = canvas.texture_creator();

    // This renderer will be used to create two boxes that will
    // be copied to the canvas and then presented
    let mut box_target = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, BOX_SIZE, BOX_SIZE)
        .map_err(|e| e.to_string())?;


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
                _ => {
                    // Every other event will be printed in the terminal
                    println!("{:?}", event);
                },
            }
        }

        // The draw color has to be set before a drawing function 
        // is going to be used on the canvas. For example, if a red line is going
        // to be drawn using canvas.draw_line() then the color has to be set
        // using canvas.set_draw_color(Color::RGB(255, 0, 0))
        //
        // In this case the canvas is going to be painted white to draw the textures
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Using the canvas and the renderer a texture can be created
        // In the closure it can be specified all the operations that want to be added
        // to the texture. NOTE: even when the renderer has been modified this closure
        // won't draw the texture to the canvas. The texture has to be copied to the 
        // canvas
        canvas
            .with_texture_canvas(&mut box_target, |texture| {

                // Selecting gray color to box and painting all
                // the box with that color
                texture.set_draw_color(Color::RGB(100, 100, 100));
                texture.clear();

                // Changing color to red to draw a rectangle in the
                // texture
                texture.set_draw_color(Color::RGB(250, 0, 0));
                texture
                    .draw_rect(Rect::new(1, 1, BOX_SIZE - 2, BOX_SIZE - 2))
                    .expect("Unable to draw box");

                // Changing color to black to draw two lines that
                // cross the texture
                texture.set_draw_color(Color::RGB(0, 0, 0));
                texture
                    .draw_line(
                        Point::new(0, 0), 
                        Point::new(BOX_SIZE as i32, BOX_SIZE as i32))
                    .expect("Unable to draw line");

                texture
                    .draw_line(
                        Point::new(BOX_SIZE as i32, 0), 
                        Point::new(0, BOX_SIZE as i32))
                    .expect("Unable to draw line");

            })
            .map_err(|e| e.to_string())?;

        // In order to draw the texture, the renderer has to be copied to 
        // the canvas. This can be done with canvas.copy() or canvas.copy_ex(). 
        // The first option copies the texture to the canvas in the specified section
        // of the canvas. The second option will do the same but one can specify more
        // options while copying the texture.
        //
        // It should be noted that the destination rect doesnt have to be the 
        // same size as the created texture. In both cases the destination rectangle
        // is different to the original texture. This gives you more flexibility when
        // using the same texture multiple times
        canvas
            .copy(
                &box_target, 
                None, 
                Rect::new(10, 0, 20, 20))?;

        canvas
            .copy_ex(
                &box_target, 
                None, 
                Rect::new(50, 50, 60, 60),
                10.0, // Angle
                Point::new(0, 0), // Rotation point
                false, 
                false)?;

        canvas.present();
    }

    Ok(())
}
