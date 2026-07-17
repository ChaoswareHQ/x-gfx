/// MCU NES emulator example: demonstrates how to run x-nes and render
/// frames using x-gfx's `McuSurface` into a raw framebuffer.
///
/// On a real embedded target, the ROM would be embedded at compile time
/// and the framebuffer would come from a display driver's DMA buffer.
/// This example uses `std` for file I/O and display but keeps the
/// rendering pipeline identical to what runs on an MCU.
///
/// Usage: cargo run --example mcu --features mcu -- <rom.nes> [frames]
///
/// Example:
///   cargo run --example mcu --features mcu -- game.nes 60
use std::time::Instant;

use gfx::frame::Frame;
use gfx::mcu::McuSurface;
use gfx::palette::Palette;

use nes::bus::Bus;
use nes::cpu::CpuRp2a03;
use nes::rom::Rom;
use nes::{reset, tick};

const DISPLAY_W: u32 = 128;
const DISPLAY_H: u32 = 120;
const STRIDE: usize = (DISPLAY_W * DISPLAY_H * 4) as usize;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rom.nes> [frames]", args[0]);
        eprintln!();
        eprintln!("Example: {} game.nes 60", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let max_frames: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(60);

    let data = std::fs::read(path).expect("failed to read ROM");
    let rom = Rom::new(&data).expect("invalid iNES ROM");

    let mut cpu = CpuRp2a03::new(0);
    let mut bus = Bus::new(rom.create_mapper());
    reset(&mut cpu, &mut bus);

    let palette = Palette::nes();
    let mcu = McuSurface::new(DISPLAY_W, DISPLAY_H);
    let mut framebuffer = [0u8; STRIDE];
    let mut frame_count = 0u32;

    let start = Instant::now();

    while frame_count < max_frames {
        // Run one frame of the emulator
        while !bus.ppu.frame_complete {
            tick(&mut cpu, &mut bus);
        }
        bus.ppu.frame_complete = false;

        // Render the NES frame into the raw framebuffer using
        // x-gfx's MCU compositor (no heap, no GPU, no floats).
        // This exact call works identically on an ESP32 or RP2040.
        let frame = Frame::new(&bus.ppu.frame);
        let written = mcu.render_into(&frame, &palette, &mut framebuffer);

        // On a real MCU, you would now send `framebuffer` to the
        // display driver:  display.write(&framebuffer);
        let _ = written;

        frame_count += 1;
    }

    let elapsed = start.elapsed();
    let elapsed_s = elapsed.as_secs_f64();
    let fps = frame_count as f64 / elapsed_s;
    let p0 = u32::from_ne_bytes(framebuffer[0..4].try_into().unwrap());
    let p1 = u32::from_ne_bytes(framebuffer[4..8].try_into().unwrap());

    println!("Rendered {frame_count} frames from {path}");
    println!("  NES resolution: 256x240");
    println!("  MCU framebuffer: {DISPLAY_W}x{DISPLAY_H} ARGB ({STRIDE} bytes)");
    println!("  Pixel(0,0): 0x{p0:08X}");
    println!("  Pixel(1,0): 0x{p1:08X}");
    println!("  Time: {elapsed_s:.3}s");
    println!("  FPS:  {fps:.1}");
}
