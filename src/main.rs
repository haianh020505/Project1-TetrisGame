mod constants;
mod tetromino;
mod game;

use macroquad::prelude::*;
use constants::*;
use game::GameState;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::new();
    let mut last_move_left = 0.0;
    let mut last_move_right = 0.0;
    let move_delay = 0.15; // Delay between moves in seconds

    loop {
        let delta_time = get_frame_time();
        let current_time = get_time();

        // Handle input
        if !game_state.game_over {
            // Block input during line clear animation
            if game_state.state == game::State::Playing {
                // Movement - separate timers for left and right to prevent sticky input
                if is_key_down(KeyCode::Left) {
                    if current_time - last_move_left > move_delay {
                        game_state.move_left();
                        last_move_left = current_time;
                    }
                } else {
                    // Reset timer when key is released
                    last_move_left = 0.0;
                }
                
                if is_key_down(KeyCode::Right) {
                    if current_time - last_move_right > move_delay {
                        game_state.move_right();
                        last_move_right = current_time;
                    }
                } else {
                    // Reset timer when key is released
                    last_move_right = 0.0;
                }

                // Rotation
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::X) {
                    game_state.rotate_cw();
                }
                if is_key_pressed(KeyCode::Z) {
                    game_state.rotate_ccw();
                }

                // Hard drop
                if is_key_pressed(KeyCode::Space) {
                    game_state.hard_drop();
                }

                // Hold
                if is_key_pressed(KeyCode::C) {
                    game_state.hold_piece();
                }

                // Soft drop
                let soft_drop = is_key_down(KeyCode::Down);
                game_state.update(delta_time, soft_drop);
            } else {
                // During animation, just update without input
                game_state.update(delta_time, false);
            }
        }

        // Restart
        if is_key_pressed(KeyCode::R) {
            game_state.reset();
        }

        // Quit
        if is_key_pressed(KeyCode::Escape) {
            // Save high score before quitting
            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                GameState::save_high_score(game_state.high_score);
            }
            break;
        }

        // Draw everything
        game_state.draw();

        next_frame().await;
    }
}
