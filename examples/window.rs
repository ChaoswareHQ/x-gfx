/// Desktop NES emulator window: load a ROM, run the emulator, render via
/// x-gfx's pixel pipeline, and display via `softbuffer` + `winit`.
///
/// Usage: cargo run --example window -- <rom.nes>
///
/// Controls:
///   Z = B,  X = A,  Shift = Select,  Enter = Start,
///   Arrow keys = D-Pad
use std::num::NonZeroU32;
use std::time::{Duration, Instant};

use gfx::frame::Frame;
use gfx::palette::Palette;

use nes::bus::Bus;
use nes::cpu::CpuRp2a03;
use nes::rom::Rom;
use nes::{reset, tick};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use gilrs::{Button, Gilrs};
use ringbuf::HeapRb;
use ringbuf::traits::{Consumer, Producer, Split};
use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes};

const SCALE: u32 = 3;
const NES_FRAME_NS: u64 = 16_639_000;
const DEFAULT_ROM: &str =
    "https://github.com/NovaSquirrel/NovaTheSquirrel/releases/download/v1.0.6a/nova.nes";

fn load_rom(path_or_url: &str) -> Vec<u8> {
    if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
        eprintln!("Downloading {path_or_url}...");
        let resp = ureq::get(path_or_url)
            .call()
            .expect("failed to download ROM");
        let data = resp.into_body().read_to_vec().expect("failed to read body");
        eprintln!("Downloaded {} bytes", data.len());
        data
    } else {
        std::fs::read(path_or_url).expect("failed to read ROM file")
    }
}

struct App {
    cpu: CpuRp2a03,
    bus: Bus,
    palette: Palette,
    window: Option<std::rc::Rc<Window>>,
    ctx: Option<Context<std::rc::Rc<Window>>>,
    surface: Option<Surface<std::rc::Rc<Window>, std::rc::Rc<Window>>>,
    frame_timer: Instant,
    frame_dur: Duration,
    acc: Duration,
    gilrs: Gilrs,
    audio_stream: Option<cpal::Stream>,
    audio_tx: Option<
        ringbuf::CachingProd<std::sync::Arc<ringbuf::SharedRb<ringbuf::storage::Heap<f32>>>>,
    >,
}

impl App {
    fn new(rom_path: Option<String>) -> Self {
        let path = rom_path.as_deref().unwrap_or(DEFAULT_ROM);
        let data = load_rom(path);
        let rom = Rom::new(&data).expect("invalid iNES ROM");

        let mut cpu = CpuRp2a03::new(0);
        let mut bus = Bus::new(rom.create_mapper());
        reset(&mut cpu, &mut bus);

        Self {
            cpu,
            bus,
            palette: Palette::nes(),
            gilrs: Gilrs::new().expect("failed to initialize gilrs"),
            window: None,
            ctx: None,
            surface: None,
            frame_timer: Instant::now(),
            frame_dur: Duration::from_nanos(NES_FRAME_NS),
            acc: Duration::new(0, 0),
            audio_stream: None,
            audio_tx: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let w = 256 * SCALE;
        let h = 240 * SCALE;

        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("x-gfx NES")
                    .with_inner_size(LogicalSize::new(w, h)),
            )
            .expect("failed to create window");

        let rc = std::rc::Rc::new(window);
        let ctx = Context::new(rc.clone()).expect("failed to create softbuffer context");
        let surface = Surface::new(&ctx, rc.clone()).expect("failed to create softbuffer surface");

        self.window = Some(rc);
        self.ctx = Some(ctx);
        self.surface = Some(surface);
        self.frame_timer = Instant::now();
        event_loop.set_control_flow(ControlFlow::Poll);

        // Init audio
        if self.audio_stream.is_none() {
            let host = cpal::default_host();
            if let Some(device) = host.default_output_device() {
                if let Ok(supported) = device.default_output_config() {
                    let sample_rate = supported.sample_rate();
                    let channels = supported.channels();
                    eprintln!("Audio: {sample_rate}Hz, {channels}ch");

                    self.bus.apu.set_sample_rate(sample_rate as f64);

                    let rb = HeapRb::<f32>::new(32768);
                    let (mut prod, mut cons) = rb.split();
                    let ch = channels as usize;

                    // Pre-fill ~2 frames of silence
                    let frames_to_fill = (sample_rate as f64 / 60.0 * 2.0) as usize;
                    for _ in 0..frames_to_fill {
                        let _ = prod.try_push(0.0);
                    }

                    let config: cpal::StreamConfig = supported.into();
                    if let Ok(stream) = device.build_output_stream(
                        config,
                        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                            for frame in data.chunks_mut(ch) {
                                let s = cons.try_pop().unwrap_or(0.0);
                                for sample in frame.iter_mut() {
                                    *sample = s;
                                }
                            }
                        },
                        |e| eprintln!("audio error: {e}"),
                        None,
                    ) {
                        stream.play().ok();
                        self.audio_stream = Some(stream);
                        self.audio_tx = Some(prod);
                    }
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(surface) = &mut self.surface {
                    surface
                        .resize(
                            NonZeroU32::new(size.width.max(1)).unwrap(),
                            NonZeroU32::new(size.height.max(1)).unwrap(),
                        )
                        .unwrap();
                }
            }
            WindowEvent::KeyboardInput { event, .. } if !event.repeat => {
                let pressed = event.state == ElementState::Pressed;
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyZ) => self.bus.pad1.b = pressed,
                    PhysicalKey::Code(KeyCode::KeyX) => self.bus.pad1.a = pressed,
                    PhysicalKey::Code(KeyCode::ShiftLeft | KeyCode::ShiftRight) => {
                        self.bus.pad1.select = pressed
                    }
                    PhysicalKey::Code(KeyCode::Enter) => self.bus.pad1.start = pressed,
                    PhysicalKey::Code(KeyCode::ArrowUp) => self.bus.pad1.up = pressed,
                    PhysicalKey::Code(KeyCode::ArrowDown) => self.bus.pad1.down = pressed,
                    PhysicalKey::Code(KeyCode::ArrowLeft) => self.bus.pad1.left = pressed,
                    PhysicalKey::Code(KeyCode::ArrowRight) => self.bus.pad1.right = pressed,
                    _ => {}
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(surface) = &mut self.surface {
                    let rc = self.window.as_ref().unwrap();
                    let size = rc.inner_size();
                    let dw = size.width.max(1);
                    let dh = size.height.max(1);
                    let mut buf = vec![0u32; (dw * dh) as usize];

                    // Use x-gfx to scale and convert the NES frame
                    let frame = Frame::new(&self.bus.ppu.frame);
                    self.palette.fill_frame(frame.as_bytes(), &mut buf, dw, dh);

                    if let Ok(mut fb) = surface.buffer_mut() {
                        let slice = fb.as_mut();
                        let n = slice.len().min(buf.len());
                        slice[..n].copy_from_slice(&buf[..n]);
                        let _ = fb.present();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        self.acc += now - self.frame_timer;
        self.frame_timer = now;

        if self.acc > Duration::from_millis(100) {
            self.acc = Duration::from_millis(100);
        }

        while self.acc >= self.frame_dur {
            while !self.bus.ppu.frame_complete {
                tick(&mut self.cpu, &mut self.bus);
            }
            self.bus.ppu.frame_complete = false;
            self.acc -= self.frame_dur;

            // Push audio samples
            if let Some(tx) = &mut self.audio_tx {
                let n = self.bus.apu.sample_count;
                if n > 0 {
                    let pushed = tx.push_slice(&self.bus.apu.audio_samples[..n]);
                    if pushed < n && pushed == 0 {
                        eprintln!("audio buffer full, dropped {n} samples");
                    }
                }
                self.bus.apu.sample_count = 0;
            }
        }

        // Poll game controllers
        while let Some(gilrs::Event {
            id: _,
            event,
            time: _,
        }) = self.gilrs.next_event()
        {
            match event {
                gilrs::EventType::ButtonChanged(button, val, _) => {
                    let pressed = val > 0.5;
                    match button {
                        Button::South => self.bus.pad1.a = pressed,
                        Button::East => self.bus.pad1.b = pressed,
                        Button::West => self.bus.pad1.b = pressed,
                        Button::North => self.bus.pad1.a = pressed,
                        Button::DPadUp => self.bus.pad1.up = pressed,
                        Button::DPadDown => self.bus.pad1.down = pressed,
                        Button::DPadLeft => self.bus.pad1.left = pressed,
                        Button::DPadRight => self.bus.pad1.right = pressed,
                        Button::Select => self.bus.pad1.select = pressed,
                        Button::Start => self.bus.pad1.start = pressed,
                        Button::LeftTrigger | Button::RightTrigger => {
                            self.bus.pad1.a = pressed;
                        }
                        Button::LeftTrigger2 | Button::RightTrigger2 => {
                            self.bus.pad1.b = pressed;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }

        event_loop.set_control_flow(ControlFlow::Poll);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rom_path = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        eprintln!("No ROM provided, downloading default ROM...");
        eprintln!("  {DEFAULT_ROM}");
        None
    };

    let mut app = App::new(rom_path);
    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).expect("event loop failed");
}
