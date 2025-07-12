# Conway’s Game of Life (Rust + Raylib)

A simple Rust implementation of Conway’s Game of Life, rendered pixel‐by‐pixel via a custom `Framebuffer` built on [raylib-rs](https://crates.io/crates/raylib).  
Load any RLE‐encoded Life pattern (e.g. Greyship, Dart, OTCA Metapixel) and watch “Life within Life” in action.

---

## 📸 Demo

![Game of Life running in terminal](./media/life-demo.gif)  
*Replace this GIF with your own export once you’ve generated it.*

---

## 🚀 Features

- **Custom framebuffer**: draw individual pixels or blocks  
- **RLE loader**: drop in any `.rle` pattern and place it anywhere on the grid  
- **Random start**: initialize with a randomized population  
- **Adjustable cell size**: trade off resolution vs. speed  
- **Modular design**: easily extend with UI controls, zoom, pan, etc.

---

## 📦 Requirements

- Rust (1.60+)
- [raylib-rs](https://crates.io/crates/raylib) (`raylib` FFI)
- [rand](https://crates.io/crates/rand) (for random seeding)
- A C toolchain with `raylib` installed on your system (e.g. via `apt`, `brew`, or `vcpkg`)

---

## 🔧 Installation

1. **Clone** the repo:
   ```bash
   git clone https://github.com/yourusername/game-of-life-rust.git
   cd game-of-life-rust

# Conway’s Game of Life (Rust + Raylib)

A simple Rust implementation of Conway’s Game of Life, rendered pixel‐by‐pixel via a custom `Framebuffer` built on [raylib-rs](https://crates.io/crates/raylib).  
Load any RLE‐encoded Life pattern (e.g. Greyship, Dart, OTCA Metapixel) and watch “Life within Life” in action.

---

## 📸 Demo

![Game of Life running in terminal](showcase.gif)  

---

## 🚀 Features

- **Custom framebuffer**: draw individual pixels or blocks  
- **RLE loader**: drop in any `.rle` pattern and place it anywhere on the grid  
- **Random start**: initialize with a randomized population  
- **Adjustable cell size**: trade off resolution vs. speed  
- **Modular design**: easily extend with UI controls, zoom, pan, etc.

---

## 📦 Requirements

- Rust (1.60+)
- [raylib-rs](https://crates.io/crates/raylib) (`raylib` FFI)
- [rand](https://crates.io/crates/rand) (for random seeding)
- A C toolchain with `raylib` installed on your system (e.g. via `apt`, `brew`, or `vcpkg`)

---

## 🔧 Installation

1. **Clone** the repo:
   ```bash
   git clone https://github.com/Ultimate-Truth-Seeker/game-of-life-rust.git
   cd game-of-life-rust
   ```

2.	Install system raylib
	•	macOS (Homebrew):
   ```bash
    brew install raylib
    ```

	•	Ubuntu / Debian:
   ```bash
    sudo apt-get update
    sudo apt-get install libraylib-dev
    ```

	•	Windows: via vcpkg:
   ```bash
    vcpkg install raylib
    ```

If necessary you may need to add CMake to compile Raylib, you run the above commands and change raylib for cmake.

3.	Build and run:
   ```bash
    cargo build --release
    cargo run --release
    ```
⸻

## 🎮 Usage

By default, the app:
	1.	Opens an 800×600 window.
	2.	Initializes a random grid of live/dead cells.
	3.	Loads any RLE patterns specified in src/ (e.g. greyship.rle, dart.rle).
	4.	Advances one generation every 16 ms and renders via the framebuffer.

You can tweak:
	•	Window size & cell size in main.rs.
	•	Initial patterns by adding more include_str!("pattern.rle") + load_rle(...) calls.
	•	Simulation speed by changing the sleep duration.

⸻

## 🧩 Extending
	•	Interactive controls: Pause, reset, step one generation.
	•	Zoom & pan: Render only a subset, scale cells, or move the viewport.
	•	Statistics overlay: Live cell count, generation number.
	•	Custom color schemes: Change live/dead colors per generation, heatmaps, etc.

⸻

## 📄 License

Distributed under the MIT License. See LICENSE for details.

⸻

Built with ❤️ in Rust + Raylib
© 2025 UTS