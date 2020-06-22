use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 16;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Shapes {
    ShapeS,
    ShapeZ,
    ShapeT,
    ShapeL,
    ShapeR,
    ShapeE, // Represents empty cell
}

pub struct Tetris {
    pub board: Vec<Vec<Shapes>>,
    pub running: bool,
}

impl Tetris {
    pub fn new() -> Self {
        let mut board = Vec::new();

        for _ in 0..BOARD_HEIGHT {
            let line = vec![Shapes::ShapeE; BOARD_WIDTH];

            board.push(line);
        }

        board[2][4] = Shapes::ShapeZ;
        board[2][5] = Shapes::ShapeZ;
        board[3][5] = Shapes::ShapeZ;
        board[3][6] = Shapes::ShapeZ;

        board[0][1] = Shapes::ShapeL;
        board[0][2] = Shapes::ShapeL;
        board[0][3] = Shapes::ShapeL;
        board[0][4] = Shapes::ShapeL;

        board[7][1] = Shapes::ShapeR;
        board[7][2] = Shapes::ShapeR;
        board[8][1] = Shapes::ShapeR;
        board[8][2] = Shapes::ShapeR;

        Tetris {
            board,
            running: true,
        }
    }

    pub fn manage_events(&mut self, event_pump: &mut sdl2::EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => {
                    self.running = false;
                }
                _ => {
                    println!("{:?}", event);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        let game = Tetris::new();

        for line in game.board {
            println!("{:?}", line);
        }
    }
}
