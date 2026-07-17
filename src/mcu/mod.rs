use crate::frame::Frame;
use crate::palette::Palette;

/// Software framebuffer compositor for MCU targets.
///
/// This module provides a simple framebuffer that accepts a
/// borrowed NES frame and converts it to ARGB pixels in a
/// caller-provided buffer.  No hardware dependencies — works
/// with any display driver that accepts a `&mut [u8]` buffer.
pub struct McuSurface {
    width: u32,
    height: u32,
}

impl McuSurface {
    /// Create a new MCU surface with the given output dimensions.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Render a NES frame into the caller's output buffer.
    ///
    /// `dst` must be at least `width * height * 4` bytes long
    /// (4 bytes per pixel for ARGB8888).
    ///
    /// Returns the number of bytes written.
    pub fn render_into(&self, frame: &Frame, palette: &Palette, dst: &mut [u8]) -> usize {
        let src = frame.as_bytes();
        let w = self.width as usize;
        let h = self.height as usize;
        let required = w * h * 4;
        let n = dst.len().min(required);

        // Safety: we validated dst length above.
        let dst_u32: &mut [u32] = unsafe {
            core::slice::from_raw_parts_mut(dst.as_mut_ptr().cast::<u32>(), n / 4)
        };

        for y in 0..h {
            for x in 0..w {
                let sx = x * 256 / w;
                let sy = y * 240 / h;
                let index = src[sy * 256 + sx];
                dst_u32[y * w + x] = palette.colour(index);
            }
        }

        n
    }

    /// Current output dimensions.
    #[must_use]
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
