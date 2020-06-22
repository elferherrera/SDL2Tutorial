pub mod surfaces;
pub mod tetris;

const WINDOW_NAME: &str = "Tetris";
const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 600;

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
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let creator = canvas.texture_creator();

    // Texture manager is used to create and load all the textures that will be used
    // in the game.  It will also be used to control everything related to drawing
    // the tetris shapes and the board.
    let mut textures = surfaces::TextureManager::new(&mut canvas, &creator);
    textures.load_textures()?;

    // The tetris object will manage all the logic of the game and its events.
    let mut tetris = tetris::Tetris::new();

    let mut event_pump = ctx.event_pump()?;

    while tetris.running {
        tetris.manage_events(&mut event_pump);
        textures.draw_game(&tetris)?;
    }

    Ok(())
}
