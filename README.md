## Prerequisites

Before you begin, ensure you have installed:

- Rust toolchain (minimum version 1.70.0)
  - Install via [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Required system dependencies:
  - Linux: `sudo apt install libgtk-3-dev`

## Building from Source

1. Clone the repository:
```bash
git clone https://github.com/sxript/fprog-baseline/
cd fprog-baseline
```

2. Build the project:
```bash
cargo build --release
```

The executable will be available at `target/release/fprog-baseline`.

Alternatively run `cargo run` to launch the application immediately without executable. 

## Usage

- Launch the application:
```bash
./target/release/fprog-baseline
```

- Drawing Polygons:
  - Left-click anywhere on the canvas to place a vertex
  - As you move your mouse, you'll see a preview line from the last vertex to your current mouse position
  - Continue clicking to add more vertices to your polygon
  - Double-click to complete the current polygon
  - Once a polygon is complete, you can start drawing a new one

- Additional Features:
  - Undo (Strg + Z) / Redo (Strg + Y) ! 
