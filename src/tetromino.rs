use macroquad::prelude::*;
use crate::constants::*;
use ::rand::seq::SliceRandom;
use ::rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoType {
    pub fn color(&self) -> Color {
        match self {
            TetrominoType::I => COLOR_I,
            TetrominoType::O => COLOR_O,
            TetrominoType::T => COLOR_T,
            TetrominoType::S => COLOR_S,
            TetrominoType::Z => COLOR_Z,
            TetrominoType::J => COLOR_J,
            TetrominoType::L => COLOR_L,
        }
    }

    pub fn shape(&self) -> Vec<Vec<u8>> {
        match self {
            TetrominoType::I => vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            TetrominoType::O => vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0],
            ],
            TetrominoType::T => vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 0, 0],
            ],
            TetrominoType::S => vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 0],
                vec![1, 1, 0, 0],
                vec![0, 0, 0, 0],
            ],
            TetrominoType::Z => vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0],
            ],
            TetrominoType::J => vec![
                vec![0, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 0, 0],
            ],
            TetrominoType::L => vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 0, 0],
            ],
        }
    }

    pub fn all_types() -> Vec<TetrominoType> {
        vec![
            TetrominoType::I,
            TetrominoType::O,
            TetrominoType::T,
            TetrominoType::S,
            TetrominoType::Z,
            TetrominoType::J,
            TetrominoType::L,
        ]
    }
}

#[derive(Clone)]
pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    pub shape: Vec<Vec<u8>>,
    pub x: i32,
    pub y: i32,
    pub rotation: u8, // 0, 1, 2, 3
}

impl Tetromino {
    pub fn new(tetromino_type: TetrominoType) -> Self {
        let shape = tetromino_type.shape();
        Tetromino {
            tetromino_type,
            shape,
            x: (GRID_WIDTH / 2) as i32 - 2,
            y: 0,
            rotation: 0,
        }
    }

    pub fn rotate_cw(&mut self) {
        // Don't rotate O piece
        if self.tetromino_type == TetrominoType::O {
            return;
        }

        let n = self.shape.len();
        let mut rotated = vec![vec![0; n]; n];

        // Rotate 90 degrees clockwise
        for i in 0..n {
            for j in 0..n {
                rotated[j][n - 1 - i] = self.shape[i][j];
            }
        }

        self.shape = rotated;
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn rotate_ccw(&mut self) {
        // Don't rotate O piece
        if self.tetromino_type == TetrominoType::O {
            return;
        }

        let n = self.shape.len();
        let mut rotated = vec![vec![0; n]; n];

        // Rotate 90 degrees counter-clockwise
        for i in 0..n {
            for j in 0..n {
                rotated[n - 1 - j][i] = self.shape[i][j];
            }
        }

        self.shape = rotated;
        self.rotation = (self.rotation + 3) % 4;
    }

    pub fn get_blocks(&self) -> Vec<(i32, i32)> {
        let mut blocks = Vec::new();
        for (i, row) in self.shape.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    blocks.push((self.x + j as i32, self.y + i as i32));
                }
            }
        }
        blocks
    }

    pub fn color(&self) -> Color {
        self.tetromino_type.color()
    }
}

// 7-Bag Randomizer
pub struct BagRandomizer {
    bag: Vec<TetrominoType>,
}

impl BagRandomizer {
    pub fn new() -> Self {
        let mut randomizer = BagRandomizer { bag: Vec::new() };
        randomizer.refill_bag();
        randomizer
    }

    fn refill_bag(&mut self) {
        self.bag = TetrominoType::all_types();
        self.bag.shuffle(&mut thread_rng());
    }

    pub fn next(&mut self) -> TetrominoType {
        if self.bag.is_empty() {
            self.refill_bag();
        }
        self.bag.pop().unwrap()
    }

    pub fn peek(&mut self) -> TetrominoType {
        if self.bag.is_empty() {
            self.refill_bag();
        }
        *self.bag.last().unwrap()
    }
}
