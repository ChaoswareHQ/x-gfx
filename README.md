# x-gfx

A minimal NES graphics output library, targeting everything from
microcontrollers to modern desktops.

## Features

- **`gpu`** (default) — GPU-accelerated output via wgpu + winit.
- **`mcu`** — Software framebuffer compositor, no hardware dependencies.
- **`ffi`** — C-compatible API for FFI / bindings.

## Usage

```toml
[dependencies]
x-gfx = { path = "../x-gfx", features = ["gpu"] }
```

See `examples/` for more.
