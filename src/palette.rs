/// NES colour palette — converts 6-bit colour indices to RGBA.
///
/// This is the standard NES palette derived from the PPU's internal
/// circuitry.  Each entry is a 32-bit ARGB value (0xAARRGGBB).
#[derive(Clone, Copy)]
pub struct Palette {
    colours: [u32; 64],
}

impl Palette {
    /// Standard NES colour palette.
    #[must_use]
    pub const fn nes() -> Self {
        Self {
            colours: [
                0xFF545454, 0xFF001E74, 0xFF080090, 0xFF440088, 0xFF7C005C, 0xFFA4001C, 0xFFA80000,
                0xFF880000, 0xFF5C2800, 0xFF284400, 0xFF005400, 0xFF005030, 0xFF004444, 0xFF000000,
                0xFF000000, 0xFF000000, 0xFFB4B4B4, 0xFF0C54C4, 0xFF303CD8, 0xFF742CC4, 0xFFAC1898,
                0xFFD8004C, 0xFFDC0800, 0xFFBC3000, 0xFF805000, 0xFF486800, 0xFF107800, 0xFF007444,
                0xFF00686C, 0xFF000000, 0xFF000000, 0xFF000000, 0xFFFCFCFC, 0xFF64B0FC, 0xFF9090FC,
                0xFFC87CFC, 0xFFFC74FC, 0xFFFC74B8, 0xFFFC7870, 0xFFFC9838, 0xFFF0B800, 0xFFBCD000,
                0xFF84DC48, 0xFF58D878, 0xFF44D0A8, 0xFF000000, 0xFF000000, 0xFF000000, 0xFFFCFCFC,
                0xFFC0E4FC, 0xFFD0D4FC, 0xFFE8CCFC, 0xFFFCC8FC, 0xFFFCC4E0, 0xFFC8B8, 0xFFFCD4A0,
                0xFFFCE090, 0xFFE4EC88, 0xFFC8F090, 0xFFA8F0A8, 0xFFB0ECC8, 0xFF000000, 0xFF000000,
                0xFF000000,
            ],
        }
    }

    /// Convert a NES palette index to a 32-bit ARGB colour.
    #[inline(always)]
    #[must_use]
    pub fn colour(&self, index: u8) -> u32 {
        self.colours[(index & 0x3F) as usize]
    }

    /// Convert an entire NES frame in-place to ARGB pixels.
    ///
    /// `src` is 61440 palette-indexed bytes (256×240).
    /// `dst` is `width * height` ARGB pixels (will be scaled).
    pub fn fill_frame(&self, src: &[u8], dst: &mut [u32], width: u32, height: u32) {
        for y in 0..height {
            for x in 0..width {
                let sx = x * 256 / width;
                let sy = y * 240 / height;
                let index = src[(sy * 256 + sx) as usize];
                dst[(y * width + x) as usize] = self.colour(index);
            }
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::nes()
    }
}
