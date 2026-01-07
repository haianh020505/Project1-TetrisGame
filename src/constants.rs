use macroquad::prelude::*;

// Kích thước lưới
pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;
pub const BLOCK_SIZE: f32 = 30.0;

// Kích thước màn hình
pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 650.0;

// Vị trí lưới game (để căn giữa trên màn hình)
pub const GRID_OFFSET_X: f32 = 50.0;
pub const GRID_OFFSET_Y: f32 = 50.0;

// Vị trí bảng giao diện
pub const UI_OFFSET_X: f32 = GRID_OFFSET_X + (GRID_WIDTH as f32 * BLOCK_SIZE) + 50.0;
pub const UI_OFFSET_Y: f32 = 50.0;

// Thời gian trong game
pub const FAST_DROP_SPEED: f32 = 0.05; // giây mỗi hàng khi giữ xuống
pub const LOCK_DELAY: f32 = 0.5; // giây trước khi khối khóa
pub const LINE_CLEAR_ANIMATION_DURATION: f32 = 0.2; // giây cho animation xóa hàng
pub const MAX_LOCK_RESETS: u32 = 15; // Số lần tối đa có thể đặt lại bộ đếm thời gian khóa (ngăn chặn xoay vô hạn)

// Đường cong tốc độ trọng lực (Tetris Guidelines chuẩn)
// Trả về tốc độ rơi tính bằng giây mỗi hàng cho mỗi cấp độ
pub const GRAVITY_CURVE: &[(u32, f32)] = &[
    (1, 1.00),   // Cấp 1
    (2, 0.79),   // Cấp 2
    (3, 0.61),   // Cấp 3
    (4, 0.47),   // Cấp 4
    (5, 0.36),   // Cấp 5
    (6, 0.28),   // Cấp 6
    (7, 0.21),   // Cấp 7
    (8, 0.15),   // Cấp 8
    (9, 0.10),   // Cấp 9+
];

// Tính điểm (Tetris Guidelines chính thức)
pub const SCORE_SINGLE: u32 = 100;
pub const SCORE_DOUBLE: u32 = 300;
pub const SCORE_TRIPLE: u32 = 500;
pub const SCORE_TETRIS: u32 = 800;
pub const SCORE_SOFT_DROP: u32 = 1;  // Điểm mỗi ô (thả chậm)
pub const SCORE_HARD_DROP: u32 = 2;  // Điểm mỗi ô (thả nhanh)
pub const SCORE_COMBO_BONUS: u32 = 50; // Điểm thưởng mỗi cấp combo * cấp độ

// Màu sắc
pub const COLOR_BACKGROUND: Color = Color::new(0.1, 0.1, 0.12, 1.0);
pub const COLOR_GRID: Color = Color::new(0.2, 0.2, 0.25, 1.0);
pub const COLOR_TEXT: Color = Color::new(0.9, 0.9, 0.9, 1.0);
pub const COLOR_GHOST: f32 = 0.3; // Giá trị alpha cho khối bóng ma

// Màu sắc các khối tetromino
pub const COLOR_I: Color = Color::new(0.0, 0.9, 0.9, 1.0);    // Xanh lơ
pub const COLOR_O: Color = Color::new(0.9, 0.9, 0.0, 1.0);    // Vàng
pub const COLOR_T: Color = Color::new(0.7, 0.0, 0.9, 1.0);    // Tím
pub const COLOR_S: Color = Color::new(0.0, 0.9, 0.0, 1.0);    // Xanh lá
pub const COLOR_Z: Color = Color::new(0.9, 0.0, 0.0, 1.0);    // Đỏ
pub const COLOR_J: Color = Color::new(0.0, 0.0, 0.9, 1.0);    // Xanh dương
pub const COLOR_L: Color = Color::new(0.9, 0.5, 0.0, 1.0);    // Cam

// File điểm cao
pub const HIGHSCORE_FILE: &str = "highscore.txt";
