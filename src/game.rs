use macroquad::prelude::*;
use crate::constants::*;
use crate::tetromino::{Tetromino, TetrominoType, BagRandomizer};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Playing,
    LineClearAnimation,
}

pub struct GameState {
    pub state: State,
    pub line_clear_timer: f32,
    pub lines_being_cleared: Vec<usize>,
    pub grid: Vec<Vec<Option<Color>>>,
    pub current_piece: Tetromino,
    pub next_piece: TetrominoType,
    pub held_piece: Option<TetrominoType>,
    pub can_hold: bool,
    pub score: u32,
    pub high_score: u32,
    pub level: u32,
    pub lines_cleared: u32,
    pub game_over: bool,
    pub fall_timer: f32,
    pub lock_timer: f32,
    pub is_on_ground: bool,
    pub bag_randomizer: BagRandomizer,
}

impl GameState {
    pub fn new() -> Self {
        let mut bag_randomizer = BagRandomizer::new();
        let current_type = bag_randomizer.next();
        let next_type = bag_randomizer.peek();
        let high_score = Self::load_high_score();

        GameState {
            state: State::Playing,
            line_clear_timer: 0.0,
            lines_being_cleared: Vec::new(),
            grid: vec![vec![None; GRID_WIDTH]; GRID_HEIGHT],
            current_piece: Tetromino::new(current_type),
            next_piece: next_type,
            held_piece: None,
            can_hold: true,
            score: 0,
            high_score,
            level: 1,
            lines_cleared: 0,
            game_over: false,
            fall_timer: 0.0,
            lock_timer: 0.0,
            is_on_ground: false,
            bag_randomizer,
        }
    }

    pub fn update(&mut self, delta_time: f32, soft_drop: bool) {
        if self.game_over {
            return;
        }

        // Handle line clear animation
        if self.state == State::LineClearAnimation {
            self.line_clear_timer += delta_time;
            
            if self.line_clear_timer >= LINE_CLEAR_ANIMATION_DURATION {
                // Animation finished, actually clear the lines
                self.complete_line_clear();
                self.state = State::Playing;
                self.line_clear_timer = 0.0;
                self.lines_being_cleared.clear();
            }
            return;
        }

        let fall_speed = if soft_drop {
            FAST_DROP_SPEED
        } else {
            INITIAL_FALL_SPEED / (self.level as f32)
        };

        self.fall_timer += delta_time;

        // Check if piece is on ground
        let was_on_ground = self.is_on_ground;
        self.is_on_ground = self.check_collision(0, 1);

        if self.is_on_ground {
            self.lock_timer += delta_time;
            if self.lock_timer >= LOCK_DELAY {
                self.lock_piece();
            }
        } else {
            self.lock_timer = 0.0;
        }

        // Auto-fall
        if self.fall_timer >= fall_speed {
            self.fall_timer = 0.0;
            if !self.is_on_ground {
                self.current_piece.y += 1;
                // Award points for soft drop
                if soft_drop {
                    self.score += SCORE_SOFT_DROP;
                }
            }
        }

        // Reset lock timer if piece was moved off ground
        if was_on_ground && !self.is_on_ground {
            self.lock_timer = 0.0;
        }
    }

    pub fn move_left(&mut self) {
        if !self.check_collision(-1, 0) {
            self.current_piece.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if !self.check_collision(1, 0) {
            self.current_piece.x += 1;
        }
    }

    pub fn rotate_cw(&mut self) {
        let mut test_piece = self.current_piece.clone();
        test_piece.rotate_cw();

        // Try direct rotation
        if !self.check_collision_piece(&test_piece) {
            self.current_piece = test_piece;
            return;
        }

        // Wall kick attempts (basic SRS-inspired)
        let kicks = vec![(1, 0), (-1, 0), (0, -1), (1, -1), (-1, -1)];
        for (dx, dy) in kicks {
            test_piece.x = self.current_piece.x + dx;
            test_piece.y = self.current_piece.y + dy;
            if !self.check_collision_piece(&test_piece) {
                self.current_piece = test_piece;
                return;
            }
        }
    }

    pub fn rotate_ccw(&mut self) {
        let mut test_piece = self.current_piece.clone();
        test_piece.rotate_ccw();

        // Try direct rotation
        if !self.check_collision_piece(&test_piece) {
            self.current_piece = test_piece;
            return;
        }

        // Wall kick attempts
        let kicks = vec![(1, 0), (-1, 0), (0, -1), (1, -1), (-1, -1)];
        for (dx, dy) in kicks {
            test_piece.x = self.current_piece.x + dx;
            test_piece.y = self.current_piece.y + dy;
            if !self.check_collision_piece(&test_piece) {
                self.current_piece = test_piece;
                return;
            }
        }
    }

    pub fn hard_drop(&mut self) {
        let ghost_y = self.calculate_ghost_y();
        let drop_distance = ghost_y - self.current_piece.y;
        
        // Award points for hard drop (2 points per cell)
        self.score += (drop_distance as u32) * SCORE_HARD_DROP;
        
        self.current_piece.y = ghost_y;
        self.lock_piece();
    }

    pub fn hold_piece(&mut self) {
        if !self.can_hold {
            return;
        }

        let current_type = self.current_piece.tetromino_type;
        
        if let Some(held_type) = self.held_piece {
            // Swap with held piece
            self.current_piece = Tetromino::new(held_type);
            self.held_piece = Some(current_type);
        } else {
            // Hold current and spawn next
            self.held_piece = Some(current_type);
            self.spawn_next_piece();
        }

        self.can_hold = false;
    }

    fn check_collision(&self, dx: i32, dy: i32) -> bool {
        let blocks = self.current_piece.get_blocks();
        for (x, y) in blocks {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x < 0 || new_x >= GRID_WIDTH as i32 || new_y >= GRID_HEIGHT as i32 {
                return true;
            }

            if new_y >= 0 && self.grid[new_y as usize][new_x as usize].is_some() {
                return true;
            }
        }
        false
    }

    fn check_collision_piece(&self, piece: &Tetromino) -> bool {
        let blocks = piece.get_blocks();
        for (x, y) in blocks {
            if x < 0 || x >= GRID_WIDTH as i32 || y >= GRID_HEIGHT as i32 {
                return true;
            }

            if y >= 0 && self.grid[y as usize][x as usize].is_some() {
                return true;
            }
        }
        false
    }

    pub fn calculate_ghost_y(&self) -> i32 {
        let mut ghost_y = self.current_piece.y;
        
        let mut test_piece = self.current_piece.clone();
        while !self.check_collision_piece(&test_piece) {
            ghost_y = test_piece.y;
            test_piece.y += 1;
        }

        ghost_y
    }

    fn lock_piece(&mut self) {
        let blocks = self.current_piece.get_blocks();
        let color = self.current_piece.color();

        for (x, y) in blocks {
            if y >= 0 && y < GRID_HEIGHT as i32 && x >= 0 && x < GRID_WIDTH as i32 {
                self.grid[y as usize][x as usize] = Some(color);
            }
        }

        self.clear_lines();
        self.spawn_next_piece();
        self.can_hold = true;
        self.lock_timer = 0.0;
        self.is_on_ground = false;
    }

    fn spawn_next_piece(&mut self) {
        let next_type = self.bag_randomizer.next();
        self.current_piece = Tetromino::new(next_type);
        self.next_piece = self.bag_randomizer.peek();
        self.fall_timer = 0.0;

        // Check if game over (piece can't spawn)
        if self.check_collision_piece(&self.current_piece) {
            self.game_over = true;
            if self.score > self.high_score {
                self.high_score = self.score;
                Self::save_high_score(self.high_score);
            }
        }
    }

    fn clear_lines(&mut self) {
        let mut lines_to_clear = Vec::new();

        for y in 0..GRID_HEIGHT {
            if self.grid[y].iter().all(|cell| cell.is_some()) {
                lines_to_clear.push(y);
            }
        }

        if lines_to_clear.is_empty() {
            return;
        }

        // Start the line clear animation
        self.lines_being_cleared = lines_to_clear;
        self.state = State::LineClearAnimation;
        self.line_clear_timer = 0.0;
    }

    fn complete_line_clear(&mut self) {
        if self.lines_being_cleared.is_empty() {
            return;
        }

        let num_lines = self.lines_being_cleared.len();

        // Sort lines in descending order and remove them from highest to lowest
        let mut sorted_lines = self.lines_being_cleared.clone();
        sorted_lines.sort_by(|a, b| b.cmp(a));
        
        for &y in sorted_lines.iter() {
            self.grid.remove(y);
        }

        // Add empty lines at the top
        for _ in 0..num_lines {
            self.grid.insert(0, vec![None; GRID_WIDTH]);
        }

        // Update score
        let lines_count = self.lines_being_cleared.len() as u32;
        self.lines_cleared += lines_count;
        
        let base_score = match lines_count {
            1 => SCORE_SINGLE,
            2 => SCORE_DOUBLE,
            3 => SCORE_TRIPLE,
            4 => SCORE_TETRIS,
            _ => 0,
        };

        self.score += base_score * self.level;

        // Update level (every 10 lines)
        self.level = (self.lines_cleared / 10) + 1;

        // Update high score
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn draw(&self) {
        // Draw background
        clear_background(COLOR_BACKGROUND);

        // Draw grid
        self.draw_grid();

        // Draw locked pieces
        self.draw_locked_pieces();

        // Draw ghost piece
        self.draw_ghost_piece();

        // Draw current piece
        self.draw_current_piece();

        // Draw UI
        self.draw_ui();

        // Draw game over screen
        if self.game_over {
            self.draw_game_over();
        }
    }

    fn draw_grid(&self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let px = GRID_OFFSET_X + x as f32 * BLOCK_SIZE;
                let py = GRID_OFFSET_Y + y as f32 * BLOCK_SIZE;
                
                draw_rectangle_lines(px, py, BLOCK_SIZE, BLOCK_SIZE, 1.0, COLOR_GRID);
            }
        }

        // Draw border
        draw_rectangle_lines(
            GRID_OFFSET_X - 2.0,
            GRID_OFFSET_Y - 2.0,
            GRID_WIDTH as f32 * BLOCK_SIZE + 4.0,
            GRID_HEIGHT as f32 * BLOCK_SIZE + 4.0,
            2.0,
            COLOR_TEXT,
        );
    }

    fn draw_locked_pieces(&self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if let Some(mut color) = self.grid[y][x] {
                    let px = GRID_OFFSET_X + x as f32 * BLOCK_SIZE;
                    let py = GRID_OFFSET_Y + y as f32 * BLOCK_SIZE;
                    
                    // Check if this row is being cleared and apply animation
                    if self.state == State::LineClearAnimation && self.lines_being_cleared.contains(&y) {
                        // Calculate animation progress (0.0 to 1.0)
                        let progress = self.line_clear_timer / LINE_CLEAR_ANIMATION_DURATION;
                        
                        // Fade out: reduce opacity
                        color.a = 1.0 - progress;
                        
                        // Shrink: reduce size toward center
                        let shrink_amount = progress * (BLOCK_SIZE - 2.0) * 0.5;
                        let block_size = (BLOCK_SIZE - 2.0) - (progress * (BLOCK_SIZE - 2.0));
                        
                        // Center the shrinking block
                        let offset = shrink_amount;
                        
                        draw_rectangle(
                            px + 1.0 + offset,
                            py + 1.0 + offset,
                            block_size,
                            block_size,
                            color
                        );
                        
                        // Draw outline with fading
                        let mut outline_color = WHITE;
                        outline_color.a = 1.0 - progress;
                        draw_rectangle_lines(
                            px + offset,
                            py + offset,
                            block_size + 2.0,
                            block_size + 2.0,
                            2.0,
                            outline_color
                        );
                    } else {
                        // Normal rendering
                        draw_rectangle(px + 1.0, py + 1.0, BLOCK_SIZE - 2.0, BLOCK_SIZE - 2.0, color);
                        draw_rectangle_lines(px, py, BLOCK_SIZE, BLOCK_SIZE, 2.0, WHITE);
                    }
                }
            }
        }
    }

    fn draw_ghost_piece(&self) {
        let ghost_y = self.calculate_ghost_y();
        let blocks = self.current_piece.get_blocks();
        let mut color = self.current_piece.color();
        color.a = COLOR_GHOST;

        let y_offset = ghost_y - self.current_piece.y;

        for (x, y) in blocks {
            if y + y_offset >= 0 {
                let px = GRID_OFFSET_X + x as f32 * BLOCK_SIZE;
                let py = GRID_OFFSET_Y + (y + y_offset) as f32 * BLOCK_SIZE;
                
                draw_rectangle(px + 1.0, py + 1.0, BLOCK_SIZE - 2.0, BLOCK_SIZE - 2.0, color);
                draw_rectangle_lines(px, py, BLOCK_SIZE, BLOCK_SIZE, 1.0, color);
            }
        }
    }

    fn draw_current_piece(&self) {
        let blocks = self.current_piece.get_blocks();
        let color = self.current_piece.color();

        for (x, y) in blocks {
            if y >= 0 {
                let px = GRID_OFFSET_X + x as f32 * BLOCK_SIZE;
                let py = GRID_OFFSET_Y + y as f32 * BLOCK_SIZE;
                
                draw_rectangle(px + 1.0, py + 1.0, BLOCK_SIZE - 2.0, BLOCK_SIZE - 2.0, color);
                draw_rectangle_lines(px, py, BLOCK_SIZE, BLOCK_SIZE, 2.0, WHITE);
            }
        }
    }

    fn draw_ui(&self) {
        let ui_x = UI_OFFSET_X;
        let mut ui_y = UI_OFFSET_Y;

        // Score
        draw_text("SCORE", ui_x, ui_y, 24.0, COLOR_TEXT);
        ui_y += 30.0;
        draw_text(&format!("{}", self.score), ui_x, ui_y, 32.0, WHITE);
        ui_y += 50.0;

        // High Score
        draw_text("HIGH SCORE", ui_x, ui_y, 24.0, COLOR_TEXT);
        ui_y += 30.0;
        draw_text(&format!("{}", self.high_score), ui_x, ui_y, 32.0, WHITE);
        ui_y += 50.0;

        // Level
        draw_text("LEVEL", ui_x, ui_y, 24.0, COLOR_TEXT);
        ui_y += 30.0;
        draw_text(&format!("{}", self.level), ui_x, ui_y, 32.0, WHITE);
        ui_y += 50.0;

        // Lines
        draw_text("LINES", ui_x, ui_y, 24.0, COLOR_TEXT);
        ui_y += 30.0;
        draw_text(&format!("{}", self.lines_cleared), ui_x, ui_y, 32.0, WHITE);
        ui_y += 50.0;

        // Next piece
        draw_text("NEXT", ui_x, ui_y, 24.0, COLOR_TEXT);
        ui_y += 30.0;
        self.draw_preview_piece(self.next_piece, ui_x, ui_y);
        ui_y += 120.0;

        // Hold piece
        draw_text("HOLD", ui_x, ui_y, 24.0, COLOR_TEXT);
        ui_y += 30.0;
        if let Some(held_type) = self.held_piece {
            self.draw_preview_piece(held_type, ui_x, ui_y);
        }

        // Controls
        ui_y += 120.0;
        draw_text("CONTROLS", ui_x, ui_y, 20.0, COLOR_TEXT);
        ui_y += 25.0;
        draw_text("← → Move", ui_x, ui_y, 16.0, COLOR_TEXT);
        ui_y += 20.0;
        draw_text("↓ Soft Drop", ui_x, ui_y, 16.0, COLOR_TEXT);
        ui_y += 20.0;
        draw_text("Space Hard Drop", ui_x, ui_y, 16.0, COLOR_TEXT);
        ui_y += 20.0;
        draw_text("Z/X Rotate", ui_x, ui_y, 16.0, COLOR_TEXT);
        ui_y += 20.0;
        draw_text("C Hold", ui_x, ui_y, 16.0, COLOR_TEXT);
        ui_y += 20.0;
        draw_text("R Restart", ui_x, ui_y, 16.0, COLOR_TEXT);
    }

    fn draw_preview_piece(&self, piece_type: TetrominoType, x: f32, y: f32) {
        let shape = piece_type.shape();
        let color = piece_type.color();
        let preview_size = 20.0;

        for (i, row) in shape.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    let px = x + j as f32 * preview_size;
                    let py = y + i as f32 * preview_size;
                    
                    draw_rectangle(px + 1.0, py + 1.0, preview_size - 2.0, preview_size - 2.0, color);
                    draw_rectangle_lines(px, py, preview_size, preview_size, 1.0, WHITE);
                }
            }
        }
    }

    fn draw_game_over(&self) {
        let overlay_color = Color::new(0.0, 0.0, 0.0, 0.7);
        draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, overlay_color);

        let text = "GAME OVER";
        let text_size = 48.0;
        let text_dims = measure_text(text, None, text_size as u16, 1.0);
        draw_text(
            text,
            (SCREEN_WIDTH - text_dims.width) / 2.0,
            SCREEN_HEIGHT / 2.0 - 50.0,
            text_size,
            WHITE,
        );

        let restart_text = "Press R to Restart";
        let restart_size = 24.0;
        let restart_dims = measure_text(restart_text, None, restart_size as u16, 1.0);
        draw_text(
            restart_text,
            (SCREEN_WIDTH - restart_dims.width) / 2.0,
            SCREEN_HEIGHT / 2.0 + 20.0,
            restart_size,
            COLOR_TEXT,
        );
    }

    fn load_high_score() -> u32 {
        fs::read_to_string(HIGHSCORE_FILE)
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0)
    }

    fn save_high_score(score: u32) {
        let _ = fs::write(HIGHSCORE_FILE, score.to_string());
    }

    pub fn reset(&mut self) {
        *self = GameState::new();
    }
}
