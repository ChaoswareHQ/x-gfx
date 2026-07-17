/// A borrowed NES frame buffer: 256×240 palette-indexed pixels.
///
/// The NES PPU outputs one byte per pixel, where each byte is an
/// index into the 64-entry NES colour palette.
pub struct Frame<'a> {
    data: &'a [u8; 256 * 240],
}

impl<'a> Frame<'a> {
    /// Wrap a reference to the emulator's PPU frame buffer.
    #[inline(always)]
    #[must_use]
    pub fn new(data: &'a [u8; 256 * 240]) -> Self {
        Self { data }
    }

    /// Width in pixels (always 256).
    #[inline(always)]
    #[must_use]
    pub const fn width(&self) -> u32 {
        256
    }

    /// Height in pixels (always 240).
    #[inline(always)]
    #[must_use]
    pub const fn height(&self) -> u32 {
        240
    }

    /// Raw palette-indexed pixel data.
    #[inline(always)]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.data
    }

    /// Get the palette index at a pixel coordinate.
    #[inline(always)]
    #[must_use]
    pub fn pixel(&self, x: u32, y: u32) -> u8 {
        self.data[y as usize * 256 + x as usize]
    }
}
