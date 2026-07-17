#![allow(
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    dead_code
)]

use crate::palette::Palette;

/// Convert a NES frame buffer to ARGB pixels using the standard palette.
///
/// # Safety
///
/// - `src` must point to a valid buffer of at least 61440 bytes (256×240).
/// - `dst` must point to a valid buffer of at least `width * height * 4` bytes.
/// - `src` and `dst` must not overlap.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gfx_render_frame(
    src: *const u8,
    dst: *mut u8,
    width: u32,
    height: u32,
) {
    let src = unsafe { core::slice::from_raw_parts(src, 256 * 240) };
    let n = (width * height * 4) as usize;
    let dst = unsafe { core::slice::from_raw_parts_mut(dst, n) };
    let dst_u32: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(dst.as_mut_ptr().cast::<u32>(), n / 4) };
    let palette = Palette::nes();

    palette.fill_frame(src, dst_u32, width, height);
}
