/// Headless NES emulator: load a ROM, run for a set number of frames,
/// render each frame to an ARGB buffer using x-gfx.
///
/// Usage: cargo run --example main -- <rom.nes> [frames]
///
/// Examples:
///   cargo run --example main -- game.nes
///   cargo run --example main -- game.nes 120
use std::time::Instant;

use gfx::frame::Frame;
use gfx::palette::Palette;

use nes::bus::Bus;
use nes::cpu::CpuRp2a03;
use nes::rom::Rom;
use nes::{reset, tick};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rom.nes> [frames]", args[0]);
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
    let mut output = vec![0u32; 256 * 240];
    let mut frame_count = 0u32;

    let start = Instant::now();

    while frame_count < max_frames {
        // Run one frame
        while !bus.ppu.frame_complete {
            tick(&mut cpu, &mut bus);
        }
        bus.ppu.frame_complete = false;

        // Render the NES frame to ARGB using x-gfx
        let frame = Frame::new(&bus.ppu.frame);
        palette.fill_frame(frame.as_bytes(), &mut output, 256, 240);

        frame_count += 1;
    }

    let elapsed = start.elapsed();
    let fps = frame_count as f64 / elapsed.as_secs_f64();

    let total_pixels = output.len();
    let elapsed_s = elapsed.as_secs_f64();

    println!("Rendered {frame_count} frames from {path}");
    println!("  Resolution: 256×240");
    println!("  Output: 256×240 ARGB pixels ({total_pixels} total)");
    println!("  First pixel: 0x{:08X}", output[0]);
    println!("  Time: {elapsed_s:.3}s");
    println!("  FPS:  {fps:.1}");
}
