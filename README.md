# x-gfx

[![Crates.io Version](https://img.shields.io/crates/v/x-gfx)](https://crates.io/crates/x-gfx)
[![Crates.io Downloads](https://img.shields.io/crates/d/x-gfx)](https://crates.io/crates/x-gfx)
[![docs.rs](https://img.shields.io/docsrs/x-gfx)](https://docs.rs/x-gfx)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](#license)
[![Rust](https://img.shields.io/badge/rust-1.97%2B-orange)](https://www.rust-lang.org)

A lightweight graphics output library written in Rust, designed to run
everywhere — from microcontrollers to modern desktops. Takes a
palette-indexed framebuffer and renders it to the screen via either
GPU-accelerated (wgpu) or software (MCU) backends.

---

## Features

- **Dual backend** — GPU-accelerated via `wgpu` on desktop, software compositor on MCU
- **Palette-indexed input** — accepts 256×240 index buffers, converts via any colour lookup table
- **Hardware scaling** — bilinear or nearest-neighbour scaling to any output resolution
- **`no_std`-compatible** — the `mcu` backend has zero dependencies
- **Dual library output** — `lib`, `cdylib` (shared), and `staticlib` for flexible integration
- **C-compatible FFI** — exposes `gfx_*` functions for easy embedding (optional, via `ffi` feature)
- **Tiny footprint** — ~150 KB shared library (fully stripped)

## Building

```sh
# Default desktop build (GPU backend via wgpu)
cargo build --release

# MCU build (software framebuffer only)
cargo build --release --no-default-features --features mcu

# Static library for embedded targets
cargo build --target thumbv7em-none-eabihf --release --no-default-features

# Example: render headless frames
cargo run --release --example main -- "your-rom.nes"

# Example: desktop window with audio + input
cargo run --release --example window -- "your-rom.nes"
```

## Usage

### As a Rust crate (via crates.io)

```toml
[dependencies]
x-gfx = "0.1"
```

### Basic rendering loop

```rust
use gfx::frame::Frame;
use gfx::palette::Palette;

let frame_data: &[u8; 256 * 240] = /* palette-indexed pixels */;
let frame = Frame::new(frame_data);
let palette = Palette::nes();

let mut output = vec![0u32; 256 * 240];
palette.fill_frame(frame.as_bytes(), &mut output, 256, 240);
```

### MCU (bare-metal) rendering

```rust
use gfx::mcu::McuSurface;

let mut framebuffer = [0u8; 128 * 120 * 4];
let mcu = McuSurface::new(128, 120);
mcu.render_into(&frame, &palette, &mut framebuffer);
// framebuffer is now ready for your display driver
```

## Project Structure

| Module | Description |
|--------|-------------|
| `frame` | 256×240 palette-indexed frame buffer wrapper |
| `palette` | Colour palette (default NES, or custom) |
| `surface` | `Surface` trait for backend-agnostic rendering |
| `config` | Display configuration (size, vsync, title) |
| `gpu` | Desktop backend — wgpu device, swapchain, compositor (requires `gpu` feature) |
| `mcu` | MCU backend — software framebuffer compositor (requires `mcu` feature) |
| `ffi` | Optional C-compatible API (enabled by `ffi` feature) |

## License

MIT OR Apache-2.0
