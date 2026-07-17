/// Rendering configuration for x-gfx backends.
pub struct Config {
    /// Window title (desktop only, ignored on MCU).
    pub title: &'static str,
    /// Width of the output surface in pixels.
    pub width: u32,
    /// Height of the output surface in pixels.
    pub height: u32,
    /// Whether to use hardware vsync (desktop only).
    pub vsync: bool,
}

impl Config {
    /// Default NES-sized config (256×240 at 3× scale).
    #[must_use]
    pub fn nes_scaled(scale: u32) -> Self {
        Self {
            title: "x-gfx",
            width: 256 * scale,
            height: 240 * scale,
            vsync: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "x-gfx",
            width: 800,
            height: 600,
            vsync: true,
        }
    }
}
