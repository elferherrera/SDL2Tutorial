use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Rect, Point};

const WINDOW_NAME: &str = "auto";
const WINDOW_HEIGHT: u32 = 200;
const WINDOW_WIDTH: u32 = 300;
const BOX_SIZE: u32 = 50;
const STEP_SIZE: u32 = 1;

// Structure used to keep track of the position of the 
// box and its velocity direction
struct Box {
    x: i32, 
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

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

    // Context timer used to extract time information for the loop
    let mut timer = ctx.timer()?;

    let mut box_position = Box {
        x: 0,
        y: 0, 
        vel_x: 1,
        vel_y: 1,
    };

    'mainloop: loop {
        for event in ctx.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } | Event::Quit { .. } => {
                    break 'mainloop
                },
                _ => {},
            }
        }

        // The timer uses ticks to meassure the time (millisencods) that has 
        // passed since the loop started
        // Using this reference one can change the position of an object
        // in order to simulate movement
        if timer.ticks() as i32 % 1000 == 0 { 
            box_position.x += STEP_SIZE as i32 * box_position.vel_x; 
            box_position.y += STEP_SIZE as i32 * box_position.vel_y; 

            // Changing the movement direction once the object is at the 
            // horizontal edge of the window
            if box_position.x < 0 {
                box_position.vel_x = 1;

            } else if (box_position.x + BOX_SIZE as i32) > WINDOW_WIDTH as i32 {
                box_position.vel_x = -1;
            }

            // Changing the movement direction once the object is at the 
            // vertical edge of the window
            if box_position.y < 0 {
                box_position.vel_y = 1;

            } else if (box_position.y + BOX_SIZE as i32) > WINDOW_HEIGHT as i32 {
                box_position.vel_y = -1;
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        canvas
            .with_texture_canvas(&mut box_target, |texture| {
                texture.set_draw_color(Color::RGB(100, 100, 100));
                texture.clear();

                texture.set_draw_color(Color::RGB(255, 0, 0));

                // Drawing a line to show the direction of the box using the
                // direction velocity. The line starts at the middle of the 
                // box and continues all the way to the end of the box
                let x_end = if box_position.vel_x > 0 {
                    BOX_SIZE
                } else {
                    0
                };

                let y_end = if box_position.vel_y > 0 {
                    BOX_SIZE
                } else {
                    0
                };

                texture
                    .draw_line(
                        Point::new((BOX_SIZE/2) as i32, (BOX_SIZE/2) as i32),
                        Point::new(x_end as i32, y_end as i32))
                    .expect("No line drawn");

            })
            .map_err(|e| e.to_string())?;

        canvas
            .copy(
                &box_target, 
                None, 
                Rect::new(
                    box_position.x,
                    box_position.y,
                    BOX_SIZE,
                    BOX_SIZE)
                )?;

        canvas.present();
    }

    Ok(())
}
