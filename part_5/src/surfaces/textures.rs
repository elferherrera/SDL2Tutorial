use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub const BOX_SIZE: u32 = 40;

pub fn create_box<'s, 'a>(
    canvas: &'s mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    // Function to create a box texture for the board. This is the figure that will be drawn
    // and seen, when no figure is on top of the board.
    //
    let mut box_target = texture_creator
        .create_texture_target(None, BOX_SIZE, BOX_SIZE)
        .map_err(|e| e.to_string())?;

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
        })
        .map_err(|e| e.to_string())?;

    Ok(box_target)
}

// There are 5 shapes in the tetris game. Since we want each piece to be unique then
// we will use 5 different funtions to create different textures for each shape
// Each created texture will be stored in the TextureManager hashmap.
pub fn shape_s<'s, 'a>(
    canvas: &'s mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    let mut box_target = texture_creator
        .create_texture_target(None, BOX_SIZE, BOX_SIZE)
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut box_target, |texture| {
            texture.set_draw_color(Color::RGB(0, 0, 0));
            texture.clear();

            texture.set_draw_color(Color::RGB(255, 0, 0));
            texture
                .fill_rect(Rect::new(4, 4, BOX_SIZE - 8, BOX_SIZE - 8))
                .expect("Unable to draw box");
        })
        .map_err(|e| e.to_string())?;

    Ok(box_target)
}

pub fn shape_z<'s, 'a>(
    canvas: &'s mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    let mut box_target = texture_creator
        .create_texture_target(None, BOX_SIZE, BOX_SIZE)
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut box_target, |texture| {
            texture.set_draw_color(Color::RGB(0, 0, 0));
            texture.clear();

            texture.set_draw_color(Color::RGB(0, 255, 0));
            texture
                .fill_rect(Rect::new(4, 4, BOX_SIZE - 8, BOX_SIZE - 8))
                .expect("Unable to draw box");
        })
        .map_err(|e| e.to_string())?;

    Ok(box_target)
}

pub fn shape_l<'s, 'a>(
    canvas: &'s mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    let mut box_target = texture_creator
        .create_texture_target(None, BOX_SIZE, BOX_SIZE)
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut box_target, |texture| {
            texture.set_draw_color(Color::RGB(0, 0, 0));
            texture.clear();

            texture.set_draw_color(Color::RGB(0, 0, 255));
            texture
                .fill_rect(Rect::new(4, 4, BOX_SIZE - 8, BOX_SIZE - 8))
                .expect("Unable to draw box");
        })
        .map_err(|e| e.to_string())?;

    Ok(box_target)
}

pub fn shape_t<'s, 'a>(
    canvas: &'s mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    let mut box_target = texture_creator
        .create_texture_target(None, BOX_SIZE, BOX_SIZE)
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut box_target, |texture| {
            texture.set_draw_color(Color::RGB(0, 0, 0));
            texture.clear();

            texture.set_draw_color(Color::RGB(255, 0, 255));
            texture
                .fill_rect(Rect::new(4, 4, BOX_SIZE - 8, BOX_SIZE - 8))
                .expect("Unable to draw box");
        })
        .map_err(|e| e.to_string())?;

    Ok(box_target)
}

pub fn shape_r<'s, 'a>(
    canvas: &'s mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    let mut box_target = texture_creator
        .create_texture_target(None, BOX_SIZE, BOX_SIZE)
        .map_err(|e| e.to_string())?;

    canvas
        .with_texture_canvas(&mut box_target, |texture| {
            texture.set_draw_color(Color::RGB(0, 0, 0));
            texture.clear();

            texture.set_draw_color(Color::RGB(0, 255, 255));
            texture
                .fill_rect(Rect::new(4, 4, BOX_SIZE - 8, BOX_SIZE - 8))
                .expect("Unable to draw box");
        })
        .map_err(|e| e.to_string())?;

    Ok(box_target)
}
