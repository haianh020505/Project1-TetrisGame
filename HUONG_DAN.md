# 🎮 HƯỚNG DẪN CHƠI TETRIS

## 📖 Giới thiệu
Chào mừng bạn đến với trò chơi Tetris! Đây là phiên bản cổ điển của trò chơi xếp hình nổi tiếng, được viết bằng Rust với giao diện đồ họa hiện đại.

## 🎯 Mục tiêu trò chơi
- **Mục tiêu chính**: Xếp các khối tetromino rơi xuống để tạo thành các hàng ngang hoàn chỉnh
- **Điểm thưởng**: Khi một hàng được xếp đầy, nó sẽ biến mất và bạn sẽ nhận được điểm
- **Thắng thua**: Trò chơi kết thúc khi các khối chồng lên nhau đến đỉnh màn hình

## 🎮 Các phím điều khiển

### Di chuyển cơ bản
| Phím | Chức năng |
|------|-----------|
| **←** (Mũi tên trái) | Di chuyển khối sang trái |
| **→** (Mũi tên phải) | Di chuyển khối sang phải |
| **↓** (Mũi tên xuống) | Rơi nhanh (Soft Drop) - khối rơi nhanh hơn |

### Xoay khối
| Phím | Chức năng |
|------|-----------|
| **↑** (Mũi tên lên) hoặc **X** | Xoay khối theo chiều kim đồng hồ |
| **Z** | Xoay khối ngược chiều kim đồng hồ |

### Thao tác đặc biệt
| Phím | Chức năng |
|------|-----------|
| **Space** (Phím cách) | Thả nhanh (Hard Drop) - khối rơi ngay xuống vị trí cuối cùng |
| **Shift** (Trái hoặc Phải) | Giữ khối hiện tại để dùng sau (Hold) |

### Điều khiển trò chơi
| Phím | Chức năng |
|------|-----------|
| **R** | Khởi động lại trò chơi |
| **Esc** | Thoát trò chơi |

## 🧩 Các loại khối Tetromino

Trò chơi có 7 loại khối khác nhau, mỗi loại có màu sắc riêng:

1. **I** (Màu xanh cyan) - Khối hình chữ I (4 ô liên tiếp)
2. **O** (Màu vàng) - Khối hình vuông (2x2)
3. **T** (Màu tím) - Khối hình chữ T
4. **S** (Màu xanh lá) - Khối hình chữ S
5. **Z** (Màu đỏ) - Khối hình chữ Z
6. **J** (Màu xanh dương) - Khối hình chữ J
7. **L** (Màu cam) - Khối hình chữ L

## 🎨 Giao diện trò chơi

### Màn hình chính
- **Lưới chơi**: Khu vực chính 10x20 ô ở bên trái
- **Khối bóng mờ (Ghost Piece)**: Hiển thị vị trí khối sẽ rơi xuống (màu mờ)
- **Bảng điểm**: Hiển thị điểm số hiện tại và thông tin trò chơi

### Bảng thông tin (bên phải)
- **SCORE**: Điểm số hiện tại của bạn
- **HIGH SCORE**: Điểm cao nhất từng đạt được
- **LEVEL**: Cấp độ hiện tại (tăng dần theo số hàng đã xóa)
- **LINES**: Tổng số hàng đã xóa được
- **NEXT**: Khối sẽ xuất hiện tiếp theo
- **HOLD**: Khối đang được giữ (nếu có)

## 💯 Hệ thống tính điểm

Điểm số được tính dựa trên số hàng bạn xóa được trong một lần:

| Số hàng xóa | Tên gọi | Điểm cơ bản |
|------------|---------|-------------|
| 1 hàng | Single | 40 điểm × Level |
| 2 hàng | Double | 100 điểm × Level |
| 3 hàng | Triple | 300 điểm × Level |
| 4 hàng | **Tetris** | 1,200 điểm × Level |

**Lưu ý**: Điểm sẽ được nhân với cấp độ hiện tại. Cố gắng xóa nhiều hàng cùng lúc để được điểm cao hơn!

## 📈 Hệ thống cấp độ

- **Tăng cấp**: Mỗi khi bạn xóa được 10 hàng, bạn sẽ lên 1 cấp
- **Tốc độ rơi**: Khối sẽ rơi nhanh hơn khi cấp độ tăng
- **Độ khó**: Trò chơi sẽ khó hơn ở các cấp cao hơn

## 🎯 Mẹo chơi hiệu quả

### 1. Sử dụng Ghost Piece (Bóng mờ)
- Khối bóng mờ hiển thị vị trí khối sẽ rơi xuống
- Giúp bạn dễ dàng xác định vị trí chính xác trước khi thả xuống

### 2. Chức năng Hold (Giữ khối)
- Nhấn **Shift** để giữ khối hiện tại
- Bạn có thể đổi khối đang giữ với khối đang rơi
- Hữu ích khi bạn cần một loại khối khác cho chiến thuật

### 3. Ưu tiên xóa nhiều hàng
- **Tetris** (xóa 4 hàng cùng lúc) cho điểm cao nhất
- Cố gắng xếp để tạo cơ hội xóa nhiều hàng một lúc

### 4. Giữ các cột đều nhau
- Tránh tạo các "lỗ hổng" không thể lấp đầy
- Cố gắng giữ bề mặt phẳng để dễ xếp

### 5. Sử dụng Hard Drop khôn ngoan
- **Space** (Hard Drop) thả khối xuống ngay lập tức
- Tiết kiệm thời gian nhưng cần chính xác
- Kiểm tra kỹ vị trí trước khi thả

### 6. Lập kế hoạch trước
- Xem khối **NEXT** để chuẩn bị chiến thuật
- Sử dụng **HOLD** để lưu khối quan trọng

### 7. Luôn để không gian cho khối I
- Khối I là khối duy nhất có thể xóa 4 hàng (Tetris)
- Cố gắng để một cột trống cho khối I

## 🔧 Cài đặt và chạy trò chơi

### Yêu cầu hệ thống
- Hệ điều hành: Windows, macOS, hoặc Linux
- Rust compiler (nếu build từ source)

### Cách chạy
```bash
cargo run --release
```

## 🏆 Điểm cao (High Score)

- Điểm cao nhất của bạn được lưu tự động vào file `highscore.txt`
- Điểm cao sẽ được giữ nguyên ngay cả khi bạn tắt game
- Thử thách bản thân để phá vỡ kỷ lục!

