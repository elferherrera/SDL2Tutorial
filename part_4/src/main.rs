use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

const WINDOW_HEIGHT: u32 = 480;
const WINDOW_WIDTH: u32 = 640;

// This constansts should be part of the Player struct, but since they are
// not going to be changed at run time it makes sense to keep them as constants
const FILE_NAME: &str = "assets/characters.bmp";
const FRAMES_PER_ANIM: i32 = 4;
const NUMBER_CHARACTERS: i32 = 3;
const SPRITE_TILE_SIZE: u32 = 32;
const STEP_SIZE: i32 = 10;

// This example uses a bitmap located in the folder assets. The bitmap has 12 drawings that
// corresponds to different characters (NUMBER_CHARACTERS) and their possible positions
// (FRAMES_PER_ANIM). Each position is bounded by a box os 32 by 32 pixels (SPRITE_TILE_SIZE).

// For this example, all the logic is placed in a Player struct
// This will allow to move the code outside the main function and start
// thinking about modules in the game development
struct Player {
    // The struct player will store two rectangles that will be used
    // to track from where to extract the sprites that correspond to the movement
    // of the player and another rectangle to keep track of the position of the
    // player within the window. These attributes could be a Rect struct but
    // it is simpler to keep track of the data using tuples for this example
    source: (i32, i32),
    destination: (i32, i32),
    // The direction attribute is used to indicate if the sprite has to be
    // flipped. This is one of the attributes that can be used when copying the
    // texture to the canvas
    direction: bool,
    // The size_increment is used to scale the original size of the sprite. This is
    // done by creating a destination rectangle that is n times larger than the original
    // source rectangle
    size_incrememt: i32,
}

impl Player {
    fn new() -> Self {
        Player {
            source: (0, 0), 
            destination: (0, 100),
            direction: false,
            size_incrememt: 4,
        }
    }

    fn change_character(&mut self) {
        // Using the space bar one can change the character that
        // is shown in the window. Using the number of characters one can cycle
        // through the different character positions in the bitmap.

        // By updating the y coordinate in the source source box, then a different
        // section of the texture can be copied to the canvas
        self.source.1 = (self.source.1 + SPRITE_TILE_SIZE as i32) % (SPRITE_TILE_SIZE as i32 * NUMBER_CHARACTERS);
    }

    fn change_frame(&mut self) {
        // By using the same logic as change character, the movement frames are
        // selected by changing the source rectangle horizontaly
        self.source.0 = (self.source.0 + SPRITE_TILE_SIZE as i32) % (SPRITE_TILE_SIZE as i32 * FRAMES_PER_ANIM);
    }

    fn move_right(&mut self) {

        // Change to the next frame in animation
        self.change_frame();

        // The horizontal destination is controlled by changing the values X values
        // in the destination box. In order to avoid a character to be drawn outside
        // the window, the actual position is compared and the new value depends on
        // whether the character is outside the window or  not.
        self.destination.0 = if (self.destination.0 + (SPRITE_TILE_SIZE as i32 * self.size_incrememt)) > WINDOW_WIDTH as i32 {
            (WINDOW_WIDTH - (SPRITE_TILE_SIZE * self.size_incrememt as u32)) as i32
        } else {
            self.destination.0 + STEP_SIZE
        };

        // Since all the sprites are facing to the right, there is no need to change
        // the directions
        self.direction = false;
    }

    fn move_left(&mut self) {

        // Change to the next frame in animation
        self.change_frame();

        // The horizontal destination is controlled by changing the values X values
        // in the destination box. In order to avoid a character to be drawn outside
        // the window, the actual position is compared and the new value depends on
        // whether the character is outside the window or  not.
        self.destination.0 = if (self.destination.0 - self.size_incrememt as i32) < 0 {
            0
        } else {
            self.destination.0 - STEP_SIZE
        };

        // In this case it is necessary to flip the sprite
        self.direction = true;
    }

    fn move_up(&mut self) {
        self.change_frame();

        // Same logic as moving left or right, only changing the Y coordinate
        self.destination.1 = if (self.destination.1 - self.size_incrememt as i32) < 0 {
            0
        } else {
            self.destination.1 - STEP_SIZE
        };
    }

    fn move_down(&mut self) {
        self.change_frame();

        // Same logic as moving left or right, only changing the Y coordinate
        self.destination.1 = if (self.destination.1 + (SPRITE_TILE_SIZE as i32 * self.size_incrememt)) > WINDOW_HEIGHT as i32 {
            (WINDOW_HEIGHT - (SPRITE_TILE_SIZE * self.size_incrememt as u32)) as i32
        } else {
            self.destination.1 + STEP_SIZE
        };
    }

    fn create_source(&self) -> Rect {
        // Creates the source rectangle from where the canvas will extract the
        // sprites. This rectangle will be linked to the surface that is going to be
        // created with the bitmap
        Rect::new(
            self.source.0, 
            self.source.1, 
            SPRITE_TILE_SIZE, 
            SPRITE_TILE_SIZE)
    }

    fn create_destination(&self) -> Rect {
        // Creates the destination rectangle. In this destination rectangle the
        // source data will be copied. If the destination rectangle is larget than
        // the source rectangle, then the canvas will fill up the area. By using
        // this property one can play with different sizes for the same object.
        Rect::new(
            self.destination.0, 
            self.destination.1, 
            SPRITE_TILE_SIZE * self.size_incrememt as u32, 
            SPRITE_TILE_SIZE * self.size_incrememt as u32)
    }

    fn make_larger(&mut self) {
        // Just playing with the size of the character
        self.size_incrememt += 1;
    }

    fn make_smaller(&mut self) {
        self.size_incrememt = if (self.size_incrememt - 1) < 0 {
            0
        } else {
            self.size_incrememt - 1
        }
    }
}

// Function to manage all the events that happend in the window
// It makes sense to have an external function that manages these events
// because it can be linked to a general game object that keeps track of all the
// ojects and textures in the canvas
fn manage_events(running: &mut bool, event_pump: &mut sdl2::EventPump, player: &mut Player) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), ..  } => {
                *running = false;
            },
            Event::KeyDown { keycode: Some(Keycode::Space), ..  } => {
                player.change_character();

            },
            Event::KeyDown { keycode: Some(Keycode::Right), ..  } => {
                player.move_right();

            },
            Event::KeyDown { keycode: Some(Keycode::Left), ..  } => {
                player.move_left();

            },
            Event::KeyDown { keycode: Some(Keycode::Up), ..  } => {
                player.move_up();

            },
            Event::KeyDown { keycode: Some(Keycode::Down), ..  } => {
                player.move_down();

            },
            Event::KeyDown { keycode: Some(Keycode::V), ..  } => {
                player.make_larger();

            },
            Event::KeyDown { keycode: Some(Keycode::B), ..  } => {
                player.make_smaller();

            },
            _ => {}
        }
    }
}


fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let video = ctx.video()?;

    let window = video
        .window("SDL2", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new(FILE_NAME))?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string())?;

    let mut player = Player::new();
    let mut running = true;
    let mut event_pump = ctx.event_pump()?;

    while running {

        manage_events(&mut running, &mut event_pump, &mut player);

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
        canvas.clear();

        // copy the frame to the canvas
        canvas.copy_ex(
            &texture,
            player.create_source(),
            player.create_destination(),
            0.0,
            None,
            player.direction,
            false,
        )?;

        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
