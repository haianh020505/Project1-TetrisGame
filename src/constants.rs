use macroquad::prelude::*;

// Grid dimensions
pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;
pub const BLOCK_SIZE: f32 = 30.0;

// Screen dimensions
pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 650.0;

// Game grid offset (to center it on screen)
pub const GRID_OFFSET_X: f32 = 50.0;
pub const GRID_OFFSET_Y: f32 = 50.0;

// UI panel position
pub const UI_OFFSET_X: f32 = GRID_OFFSET_X + (GRID_WIDTH as f32 * BLOCK_SIZE) + 50.0;
pub const UI_OFFSET_Y: f32 = 50.0;

// Game timing
pub const INITIAL_FALL_SPEED: f32 = 1.0; // seconds per row
pub const FAST_DROP_SPEED: f32 = 0.05; // seconds per row when holding down
pub const LOCK_DELAY: f32 = 0.5; // seconds before piece locks
pub const LINE_CLEAR_ANIMATION_DURATION: f32 = 0.3; // seconds for line clear animation

// Scoring (Official Tetris Guidelines)
pub const SCORE_SINGLE: u32 = 100;
pub const SCORE_DOUBLE: u32 = 300;
pub const SCORE_TRIPLE: u32 = 500;
pub const SCORE_TETRIS: u32 = 800;
pub const SCORE_SOFT_DROP: u32 = 1;  // Points per cell (soft drop)
pub const SCORE_HARD_DROP: u32 = 2;  // Points per cell (hard drop)

// Colors
pub const COLOR_BACKGROUND: Color = Color::new(0.1, 0.1, 0.12, 1.0);
pub const COLOR_GRID: Color = Color::new(0.2, 0.2, 0.25, 1.0);
pub const COLOR_TEXT: Color = Color::new(0.9, 0.9, 0.9, 1.0);
pub const COLOR_GHOST: f32 = 0.3; // Alpha value for ghost piece

// Tetromino colors
pub const COLOR_I: Color = Color::new(0.0, 0.9, 0.9, 1.0);    // Cyan
pub const COLOR_O: Color = Color::new(0.9, 0.9, 0.0, 1.0);    // Yellow
pub const COLOR_T: Color = Color::new(0.7, 0.0, 0.9, 1.0);    // Purple
pub const COLOR_S: Color = Color::new(0.0, 0.9, 0.0, 1.0);    // Green
pub const COLOR_Z: Color = Color::new(0.9, 0.0, 0.0, 1.0);    // Red
pub const COLOR_J: Color = Color::new(0.0, 0.0, 0.9, 1.0);    // Blue
pub const COLOR_L: Color = Color::new(0.9, 0.5, 0.0, 1.0);    // Orange

// High score file
pub const HIGHSCORE_FILE: &str = "highscore.txt";
