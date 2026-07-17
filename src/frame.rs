/// A borrowed palette-indexed frame buffer: 256×240 pixels.
///
/// Each byte is an index into a colour palette (e.g. the NES 64-entry
/// palette provided by [`Palette`](crate::palette::Palette)).
pub struct Frame<'a> {
    data: &'a [u8; 256 * 240],
}

impl<'a> Frame<'a> {
    /// Wrap a reference to a 256×240 palette-indexed frame.
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
