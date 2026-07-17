use crate::frame::Frame;
use crate::palette::Palette;

/// Error type returned by surface operations.
#[derive(Debug)]
pub enum Error {
    /// The backend could not be initialised.
    Init,
    /// The surface could not be resized.
    Resize,
    /// The frame could not be presented.
    Present,
    /// The underlying platform is not available.
    Unsupported,
}

/// Common interface implemented by both GPU and MCU backends.
pub trait Surface {
    /// Render (and optionally scale) a NES frame to the display.
    ///
    /// Returns the number of raw bytes written to the output buffer,
    /// or an error if presentation failed.
    fn render(&mut self, frame: &Frame, palette: &Palette) -> Result<(), Error>;

    /// Resize the output surface.
    fn resize(&mut self, width: u32, height: u32) -> Result<(), Error>;

    /// Current output dimensions.
    fn dimensions(&self) -> (u32, u32);
}

/// Create a GPU-accelerated surface from a config.
///
/// On `gpu` targets this creates a window + wgpu surface.
#[cfg(feature = "gpu")]
pub fn create_surface(config: &crate::config::Config) -> Result<impl Surface, Error> {
    crate::gpu::context::GpuContext::new(config)
}
