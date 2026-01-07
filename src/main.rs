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
    let move_delay = 0.15; // Độ trễ giữa các lần di chuyển tính bằng giây

    loop {
        let delta_time = get_frame_time();
        let current_time = get_time();

        // Xử lý đầu vào
        if !game_state.game_over {
            // Chặn đầu vào trong khi animation xóa hàng
            if game_state.state == game::State::Playing {
                // Di chuyển - bộ đếm thời gian riêng biệt cho trái và phải để tránh đầu vào dính
                if is_key_down(KeyCode::Left) {
                    if current_time - last_move_left > move_delay {
                        game_state.move_left();
                        last_move_left = current_time;
                    }
                } else {
                    // Đặt lại bộ đếm thời gian khi phím được thả ra
                    last_move_left = 0.0;
                }
                
                if is_key_down(KeyCode::Right) {
                    if current_time - last_move_right > move_delay {
                        game_state.move_right();
                        last_move_right = current_time;
                    }
                } else {
                    // Đặt lại bộ đếm thời gian khi phím được thả ra
                    last_move_right = 0.0;
                }

                // Xoay
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::X) {
                    game_state.rotate_cw();
                }
                if is_key_pressed(KeyCode::Z) {
                    game_state.rotate_ccw();
                }

                // Thả nhanh
                if is_key_pressed(KeyCode::Space) {
                    game_state.hard_drop();
                }

                // Giữ khối
                if is_key_pressed(KeyCode::C) {
                    game_state.hold_piece();
                }

                // Thả chậm
                let soft_drop = is_key_down(KeyCode::Down);
                game_state.update(delta_time, soft_drop);
            } else {
                // Trong khi animation, chỉ cập nhật mà không có đầu vào
                game_state.update(delta_time, false);
            }
        }

        // Khởi động lại
        if is_key_pressed(KeyCode::R) {
            game_state.reset();
        }

        // Thoát
        if is_key_pressed(KeyCode::Escape) {
            // Lưu điểm cao trước khi thoát
            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                GameState::save_high_score(game_state.high_score);
            }
            break;
        }

        // Vẽ mọi thứ
        game_state.draw();

        next_frame().await;
    }
}
