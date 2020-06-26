use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

mod textures;
use crate::tetris;
use tetris::Shapes;
use textures::BOX_SIZE;

const MARGIN_X: i32 = 50;
const MARGIN_Y: i32 = 100;

const NEXT_AREA_X: i32 = 470;
const NEXT_AREA_Y: i32 = 50;
const NEXT_AREA_WIDTH: u32 = 100;
const NEXT_AREA_HEIGHT: u32 = 100;
const NEXT_AREA_BOX: u32 = 20;
const NEXT_AREA_PAD_X: i32 = 10;
const NEXT_AREA_PAD_Y: i32 = 30;

// Struct to contain all the textures that will be used during the game.
// The textures are stored like this to avoid computing them all the time
// and also to practice a bit with lifetimes and references.
//
// Also, since a mutable reference to the canvas is stored in the TextureManager
// then all the operations that will be done in the canvas has to be done
// through the TextureManager, otherwise there will be multiple mut borrows.
pub struct TextureManager<'a> {
    pub canvas: &'a mut Canvas<Window>,
    pub creator: &'a TextureCreator<WindowContext>,
    pub board_textures: HashMap<tetris::Shapes, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(canvas: &'a mut Canvas<Window>, creator: &'a TextureCreator<WindowContext>) -> Self {
        // By storing a mutable reference to the canvas, the TextureManager is now the owner of the
        // object and can do all the operations that are required to draw all the textures and
        // shapes. Also, since only one texture creator is needed to manage and create all the
        // objects, then a reference is also stored in this struct

        TextureManager {
            canvas,
            creator,
            board_textures: HashMap::new(),
        }
    }

    pub fn load_textures(&mut self) -> Result<(), String> {
        // Loading all the textures that will be used in the game this dictionary will be used to
        // avoid creating all the time the textures that corresponds to the figures and all the
        // objects that are going to be drawn in the board. A similar texture manager can be used
        // to store sprites

        self.board_textures.insert(
            Shapes::ShapeE,
            textures::create_box(self.canvas, self.creator)?,
        );
        self.board_textures.insert(
            Shapes::ShapeS,
            textures::shape_s(self.canvas, self.creator)?,
        );
        self.board_textures.insert(
            Shapes::ShapeZ,
            textures::shape_z(self.canvas, self.creator)?,
        );
        self.board_textures.insert(
            Shapes::ShapeT,
            textures::shape_t(self.canvas, self.creator)?,
        );
        self.board_textures.insert(
            Shapes::ShapeL,
            textures::shape_l(self.canvas, self.creator)?,
        );
        self.board_textures.insert(
            Shapes::ShapeR,
            textures::shape_r(self.canvas, self.creator)?,
        );

        Ok(())
    }

    pub fn draw_game(&mut self, tetris: &tetris::Tetris) -> Result<(), String> {
        // Drawing all the textures and board accordingly to the status of the game.
        // Since the board is represented by a matrix (a vector of vectors) and each
        // cell in the matrix will be drawn with a different texture in order to
        // give the ilusion of separate figures on the board

        self.canvas.set_draw_color(Color::RGB(10, 10, 10));
        self.canvas.clear();

        // Drawing the area for the next piece
        // In this rectangle the next piece is going to be shown
        self.canvas.set_draw_color(Color::RGB(200, 200, 200));
        self.canvas.fill_rect(Rect::new(
            NEXT_AREA_X,
            NEXT_AREA_Y,
            NEXT_AREA_WIDTH,
            NEXT_AREA_HEIGHT,
        ))?;

        // Drawing the next piece in the area for next piece
        for (i, line) in tetris.next_piece.states[0].iter().enumerate() {
            for (j, col) in line.iter().enumerate() {
                if *col == Shapes::ShapeE {
                    continue;
                };

                let delta_x = NEXT_AREA_BOX as i32 * j as i32;
                let delta_y = NEXT_AREA_BOX as i32 * i as i32;

                // Using the Shape enum to select the texture from the hashmap
                // This texture will be copied to a section of the board
                self.canvas.copy(
                    &self.board_textures[col],
                    None,
                    Rect::new(
                        NEXT_AREA_PAD_X + NEXT_AREA_X as i32 + delta_x,
                        NEXT_AREA_PAD_Y + NEXT_AREA_Y as i32 + delta_y,
                        NEXT_AREA_BOX,
                        NEXT_AREA_BOX,
                    ),
                )?;
            }
        }

        // Drawing the board by checking each element in the matrix. The value is matched
        // to the enum Shapes in order to draw the correct texture.
        for (i, line) in tetris.board.iter().enumerate() {
            for (j, col) in line.iter().enumerate() {
                let delta_x = BOX_SIZE as i32 * j as i32;
                let delta_y = BOX_SIZE as i32 * i as i32;

                // Using the Shape enum to select the texture from the hashmap
                // This texture will be copied to a section of the board
                self.canvas.copy(
                    &self.board_textures[col],
                    None,
                    Rect::new(MARGIN_X + delta_x, MARGIN_Y + delta_y, BOX_SIZE, BOX_SIZE),
                )?;
            }
        }

        // Drawing the current piece on top of the board. The current piece is not "stored"
        // in the board until it is located in its final position. A piece is stored when its
        // status values are copied to the board
        let current_state = tetris.current_piece.current_state;

        // Borrowing the current piece to extract all the information required
        let current_piece = &tetris.current_piece;

        for (i, line) in current_piece.states[current_state].iter().enumerate() {
            for (j, col) in line.iter().enumerate() {
                match col {
                    Shapes::ShapeE => continue,
                    _ => {
                        // Only draw a piece when the value is not empty (ShapeE)
                        let delta_x = BOX_SIZE as i32 * (j as i32 + current_piece.x_pos as i32);
                        let delta_y = BOX_SIZE as i32 * (i as i32 + current_piece.y_pos as i32);

                        // Using the Shape enum to select the texture from the hashmap
                        // This texture will be copied to a section of the board
                        self.canvas.copy(
                            &self.board_textures[col],
                            None,
                            Rect::new(MARGIN_X + delta_x, MARGIN_Y + delta_y, BOX_SIZE, BOX_SIZE),
                        )?;
                    }
                }
            }
        }

        self.canvas.present();

        Ok(())
    }
}
