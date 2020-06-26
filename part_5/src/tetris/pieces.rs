use crate::tetris::Shapes::{self, ShapeS, ShapeE, ShapeZ, ShapeT, ShapeL, ShapeR};

type Piece = Vec<Vec<Shapes>>;
type States = Vec<Piece>;

#[derive(Debug, Clone)]
pub struct TetrisPiece {
    pub states: States,
    pub x_pos: u32,
    pub y_pos: u32,
    pub current_state: usize,
}


pub trait CreatePiece {
    fn new() -> TetrisPiece;
}

pub struct TetrisS;
impl CreatePiece for TetrisS {
    fn new() -> TetrisPiece {
        TetrisPiece {
            states: vec![
                vec![
                    vec![ShapeE, ShapeS, ShapeS, ShapeE],
                    vec![ShapeS, ShapeS, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
                vec![
                    vec![ShapeS, ShapeE, ShapeE, ShapeE],
                    vec![ShapeS, ShapeS, ShapeE, ShapeE],
                    vec![ShapeE, ShapeS, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
            ],
            x_pos: 4,
            y_pos: 0,
            current_state: 0,
        }
    }
}

pub struct TetrisZ;
impl CreatePiece for TetrisZ {
    fn new() -> TetrisPiece {
        TetrisPiece {
            states: vec![
                vec![
                    vec![ShapeZ, ShapeZ, ShapeE, ShapeE],
                    vec![ShapeE, ShapeZ, ShapeZ, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
                vec![
                    vec![ShapeE, ShapeZ, ShapeE, ShapeE],
                    vec![ShapeZ, ShapeZ, ShapeE, ShapeE],
                    vec![ShapeZ, ShapeE, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
            ],
            x_pos: 4,
            y_pos: 0,
            current_state: 0,
        }
    }
}

pub struct TetrisT;
impl CreatePiece for TetrisT {
    fn new() -> TetrisPiece {
        TetrisPiece {
            states: vec![
                vec![
                    vec![ShapeT, ShapeT, ShapeT, ShapeE],
                    vec![ShapeE, ShapeT, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
                vec![
                    vec![ShapeE, ShapeT, ShapeE, ShapeE],
                    vec![ShapeT, ShapeT, ShapeE, ShapeE],
                    vec![ShapeE, ShapeT, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
                vec![
                    vec![ShapeE, ShapeT, ShapeE, ShapeE],
                    vec![ShapeT, ShapeT, ShapeT, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
                vec![
                    vec![ShapeT, ShapeE, ShapeE, ShapeE],
                    vec![ShapeT, ShapeT, ShapeE, ShapeE],
                    vec![ShapeT, ShapeE, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
            ],
            x_pos: 4,
            y_pos: 0,
            current_state: 0,
        }
    }
}

pub struct TetrisL;
impl CreatePiece for TetrisL {
    fn new() -> TetrisPiece {
        TetrisPiece {
            states: vec![
                vec![
                    vec![ShapeL, ShapeL, ShapeL, ShapeL],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
                vec![
                    vec![ShapeL, ShapeE, ShapeE, ShapeE],
                    vec![ShapeL, ShapeE, ShapeE, ShapeE],
                    vec![ShapeL, ShapeE, ShapeE, ShapeE],
                    vec![ShapeL, ShapeE, ShapeE, ShapeE],
                ],
            ],
            x_pos: 4,
            y_pos: 0,
            current_state: 0,
        }
    }
}

pub struct TetrisR;
impl CreatePiece for TetrisR {
    fn new() -> TetrisPiece {
        TetrisPiece {
            states: vec![
                vec![
                    vec![ShapeR, ShapeR, ShapeE, ShapeE],
                    vec![ShapeR, ShapeR, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                    vec![ShapeE, ShapeE, ShapeE, ShapeE],
                ],
            ],
            x_pos: 4,
            y_pos: 0,
            current_state: 0,
        }
    }
}
