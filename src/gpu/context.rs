use crate::config::Config;
use crate::frame::Frame;
use crate::palette::Palette;
use crate::surface::{Error, Surface};

/// GPU-accelerated rendering context backed by wgpu + winit.
///
/// Creates a window and manages a wgpu device for compositing
/// NES frames onto the screen with hardware scaling.
pub struct GpuContext {
    width: u32,
    height: u32,
}

impl GpuContext {
    /// Create a new GPU context and open a window.
    ///
    /// Returns `Err(Error::Init)` if the window or GPU device
    /// cannot be created.
    pub fn new(config: &Config) -> Result<Self, Error> {
        // TODO: Implement winit window creation + wgpu device init.
        let _ = config;
        Err(Error::Unsupported)
    }
}

impl Surface for GpuContext {
    fn render(&mut self, frame: &Frame, palette: &Palette) -> Result<(), Error> {
        // TODO: Upload frame to GPU texture, composite, present.
        let _ = (frame, palette);
        Err(Error::Unsupported)
    }

    fn resize(&mut self, width: u32, height: u32) -> Result<(), Error> {
        self.width = width;
        self.height = height;
        // TODO: Rebuild swapchain.
        Ok(())
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
