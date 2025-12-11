# 🎮 Tetris Game

A classic Tetris game implementation written in Rust using the Macroquad game framework.

## 📖 About

This is a modern implementation of the classic Tetris puzzle game with smooth graphics, ghost piece preview, hold functionality, and a comprehensive scoring system. Built with Rust for performance and cross-platform compatibility. 

## ✨ Features

- 🎯 **Classic Tetris Gameplay** - All 7 standard tetromino pieces
- 👻 **Ghost Piece** - Shows where the piece will land
- 🔄 **Hold System** - Save a piece for later use
- 📊 **Scoring System** - Points based on lines cleared (Single, Double, Triple, Tetris)
- 📈 **Progressive Difficulty** - Speed increases with level
- 🏆 **High Score Tracking** - Automatically saved to file
- ⚡ **Wall Kicks** - Advanced rotation system
- 🎨 **Clean UI** - Color-coded pieces and intuitive interface
- 🖥️ **Cross-Platform** - Works on Windows, macOS, and Linux

## 🎮 Controls

### Basic Movement
- **←** (Left Arrow) - Move piece left
- **→** (Right Arrow) - Move piece right
- **↓** (Down Arrow) - Soft drop (faster fall)

### Rotation
- **↑** (Up Arrow) or **X** - Rotate clockwise
- **Z** - Rotate counter-clockwise

### Special Actions
- **Space** - Hard drop (instant drop to bottom)
- **Shift** (Left or Right) - Hold current piece

### Game Controls
- **R** - Restart game
- **Esc** - Exit game

## 🧩 Tetromino Pieces

| Piece | Color | Shape |
|-------|-------|-------|
| **I** | Cyan | Straight line (4 blocks) |
| **O** | Yellow | Square (2×2) |
| **T** | Purple | T-shape |
| **S** | Green | S-shape |
| **Z** | Red | Z-shape |
| **J** | Blue | J-shape |
| **L** | Orange | L-shape |

## 💯 Scoring System

Points are calculated based on the number of lines cleared at once:

| Lines Cleared | Name | Base Points |
|---------------|------|-------------|
| 1 line | Single | 40 × Level |
| 2 lines | Double | 100 × Level |
| 3 lines | Triple | 300 × Level |
| 4 lines | **Tetris** | 1,200 × Level |

**Note:** Score is multiplied by your current level. Clear multiple lines at once for maximum points!

## 📈 Level System

- **Level Up:** Every 10 lines cleared increases your level by 1
- **Speed:** Pieces fall faster as your level increases
- **Difficulty:** Higher levels provide greater challenges and rewards

## 🚀 Installation & Running

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

### Build and Run

1. Clone the repository:
```bash
git clone https://github.com/haianh020505/Project1-TetrisGame. git
cd Project1-TetrisGame
```

2. Run the game:
```bash
cargo run --release
```

The `--release` flag ensures optimal performance. 

## 📦 Dependencies

- **[macroquad](https://github.com/not-fl3/macroquad)** (v0.4) - Cross-platform game framework
- **[rand](https://github.com/rust-random/rand)** (v0.8) - Random number generation

## 🎯 Gameplay Tips

1. **Use Ghost Piece** - The translucent piece shows where your current piece will land
2. **Master the Hold Function** - Save important pieces for strategic moments
3. **Aim for Tetris** - Clearing 4 lines at once gives the most points
4. **Keep It Flat** - Avoid creating gaps in your stack
5. **Plan Ahead** - Check the NEXT piece to strategize
6. **Save Space for I-Pieces** - Keep a column open for clearing 4 lines
7. **Use Hard Drop Wisely** - Space bar drops instantly, but be precise!

## 🎨 UI Elements

### Game Board
- **10×20 Grid** - Standard Tetris playing field
- **Ghost Piece** - Semi-transparent preview of landing position

### Info Panel (Right Side)
- **SCORE** - Your current score
- **HIGH SCORE** - Best score achieved (saved to `highscore.txt`)
- **LEVEL** - Current difficulty level
- **LINES** - Total lines cleared
- **NEXT** - Preview of upcoming piece
- **HOLD** - Currently held piece (if any)

## 📁 Project Structure

```
Project1-TetrisGame/
├── src/              # Rust source code
├── Cargo.toml        # Project dependencies
├── Cargo.lock        # Dependency lock file
├── highscore.txt     # High score storage
├── HUONG_DAN.md      # Vietnamese instructions
└── README.md         # This file
```

## 🏆 High Score

Your highest score is automatically saved to `highscore.txt` and persists between game sessions.  Challenge yourself to beat your own record!

## 🌐 Language Support

- **English** - This README
- **Vietnamese** - See [HUONG_DAN.md](HUONG_DAN.md) for detailed Vietnamese instructions

## 🛠️ Technical Details

- **Language:** Rust 🦀
- **Game Framework:** Macroquad
- **Edition:** Rust 2021
- **Platform:** Cross-platform (Windows, macOS, Linux)

## 📝 License

This project is open source.  Feel free to use, modify, and distribute as needed.

## 👨‍💻 Developer

Created by [haianh020505](https://github.com/haianh020505)

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the issues page or submit a pull request. 

## 🎊 Enjoy the Game!

Have fun playing Tetris and try to beat your high score!  If you have any questions or feedback, please open an issue on GitHub.

---

**Version:** 1.0  
**Built with:** ❤️ and Rust
