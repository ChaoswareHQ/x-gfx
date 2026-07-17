# x-gfx

A minimal graphics output library, targeting everything from
microcontrollers to modern desktops.

Used by [x-chaos](https://github.com/ChaoswareHQ/x-chaos) for
retro-emulator rendering, but general-purpose enough for any
palette-indexed framebuffer workload.

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
