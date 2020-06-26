use rand::Rng;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod pieces;
use pieces::CreatePiece;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 16;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
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
    pub current_piece: pieces::TetrisPiece,
    pub next_piece: pieces::TetrisPiece,
    pub score: u32,
}

impl Tetris {
    pub fn new() -> Self {
        // The board is represented by a vector of vectors
        // Each line in the board will be filled with an 
        // enum representing the shape
        let mut board = Vec::new();

        // Filling up the board with the enum ShapeE. This
        // enum represents an empty cell in the board
        for _ in 0..BOARD_HEIGHT {
            let line = vec![Shapes::ShapeE; BOARD_WIDTH];
            // Pushing an empty line to create and empty board
            board.push(line);
        }

        Tetris {
            board,
            running: true,
            current_piece: Tetris::random_piece(),
            next_piece: Tetris::random_piece(),
            score: 0,
        }
    }

    fn random_piece() -> pieces::TetrisPiece {
        // Generating a random piece for the board
        // This new piece will be stored as the current piece
        let num: u32 = rand::thread_rng().gen_range(0, 5);

        match num {
            0 => pieces::TetrisS::new(),
            1 => pieces::TetrisZ::new(),
            2 => pieces::TetrisT::new(),
            3 => pieces::TetrisL::new(),
            _ => pieces::TetrisR::new(),
        }
    }

    fn check_lines(&mut self) {
        // Checking if a line has been completed. If it has been completed
        // the line is deleted and a new line is added 
        //
        // Cycling through all the lines and if an empty value is found
        // the there is no point comparing the rest of the values. The outer
        // loop is continued to search in the next line
        //
        // The line numbers where a complete value was found is stored in
        // the vector. This vector will be used to remove the lines from the board
        let mut complete_lines = Vec::new();

        'outer: for (i, line) in self.board.iter().enumerate() {
            for col in line.iter() {
                if *col == Shapes::ShapeE { continue 'outer }
            }

            complete_lines.push(i);
        }

        for i in complete_lines {
            // The complete lines are removed based on the index collected before
            self.board.remove(i);

            // Then a new line is inserted at the beginnig of the board
            // This gives the ilusion of falling
            self.board.insert(0, vec![Shapes::ShapeE; BOARD_WIDTH]);

            // The game score is stored in the score variable
            self.score += 1;
        }
    }

    fn make_permanent(&mut self) {
        // Copying the current piece to the board and creating
        // a new piece for the current piece
        
        let piece = &self.current_piece.states[self.current_piece.current_state];
        for (i, line) in piece.iter().enumerate() {
            for (j, col) in line.iter().enumerate() {
                // if the piece component is empty then there is no point
                // copying the piece to the board 
                if *col == Shapes::ShapeE { continue };

                // Actual position of the state piece on the board
                let board_x = self.current_piece.x_pos as usize + j;
                let board_y = self.current_piece.y_pos as usize + i;

                // Copying the col value to the board
                self.board[board_y][board_x] = *col;
            }
        }

        // Creating a new current piece for the board
        self.current_piece = self.next_piece.clone();
        self.next_piece = Tetris::random_piece();

        // Checking if a line is complete
        self.check_lines();
    }

    fn change_state(&mut self) {
        // To change the state of the current piece one can cycle through
        // the indices of the available states in the shape type
        let current_state = self.current_piece.current_state;
        let number_states = self.current_piece.states.len();
        let new_state = (current_state + 1) % number_states;

        // The new state is checked to see if it stays within the
        // bounds and if there are no other pieces on the board
        match self.chech_new_position(
            self.current_piece.x_pos as i32,
            self.current_piece.y_pos as i32,
            new_state) {

            true => self.current_piece.current_state = new_state,
            false => self.current_piece.current_state = current_state,
        }
    }

    fn move_right(&mut self) {
        // To move the piece to the right the x_pos is increases 1 unit
        // The new state is checked to see if it stays within the
        // bounds and if there are no other pieces on the board
        match self.chech_new_position(
            self.current_piece.x_pos as i32 + 1 as i32,
            self.current_piece.y_pos as i32,
            self.current_piece.current_state) {

            true => self.current_piece.x_pos += 1,
            false => self.current_piece.x_pos += 0,
        }
    }

    fn move_left(&mut self) {
        // To move the piece to the left the x_pos is decreased 1 unit
        // The new state is checked to see if it stays within the
        // bounds and if there are no other pieces on the board
        match self.chech_new_position(
            self.current_piece.x_pos as i32 - 1 as i32,
            self.current_piece.y_pos as i32,
            self.current_piece.current_state) {

            true => self.current_piece.x_pos -= 1,
            false => self.current_piece.x_pos += 0,
        }
    }

    fn move_down(&mut self) {
        // To move the piece down the y_pos is increased 1 unit
        // The new state is checked to see if it stays within the
        // bounds and if there are no other pieces on the board
        match self.chech_new_position(
            self.current_piece.x_pos as i32,
            self.current_piece.y_pos as i32 + 1,
            self.current_piece.current_state) {

            true => self.current_piece.y_pos += 1,
            false => self.current_piece.y_pos += 0,
        }
    }

    pub fn chech_new_position(&mut self, new_x: i32, new_y: i32, new_state: usize) -> bool {
        // Checks the new probable new position of the current piece
        // It the new state or position is outsize the board or if there
        // is shape that is not empty then the new position es not allowed

        // If the new position is less than zero it means that it is outside
        // the board and it isn't allowed
        if (new_x < 0) | (new_y < 0) { return false}

        // Getting the new state of the piece
        let piece = &self.current_piece.states[new_state];

        // Checking all the components of the new state
        for (i, line) in piece.iter().enumerate() {
            for (j, col) in line.iter().enumerate() {

                // if the piece component is empty then there is no point
                // comparing it with the board
                if *col == Shapes::ShapeE { continue };

                // New position of the state piece on the board
                let board_position_x = new_x + j as i32;
                let board_position_y = new_y + i as i32;

                // If the new position is larger that the board itself then the
                // new position is not valid
                if board_position_x >= BOARD_WIDTH as i32 { return false };
                if board_position_y >= BOARD_HEIGHT as i32 { return false };

                // Compare the piece section with the board section
                // If the board is not empty in this section then it isn't allowed
                // to move the piece
                let board_value = &self.board[board_position_y as usize][board_position_x as usize];
                if *board_value != Shapes::ShapeE { return false };
            }
        }

        true
    }

    pub fn manage_events(&mut self, event_pump: &mut sdl2::EventPump) {
        // Using the SDL2 event pump all the event comming from
        // the video contex can be managed. Each event will cause
        // a change in the board
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), ..  } | Event::Quit { .. } => {
                    self.running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..  } => {
                    self.change_state();
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..  } => {
                    self.move_right();
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..  } => {
                    self.move_left();
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..  } => {
                    self.move_down();
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..  } => {
                    self.make_permanent();
                },
                _ => { 
                    //println!("{:?}", event); 
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
        // Printing the creation of the actual piece from
        // the new board
        let game = Tetris::new();

        for state in game.current_piece.states.iter() {
            println!("State");
            for line in state.iter() {
                println!("{:?}", line);
            }
        }
    }
}
