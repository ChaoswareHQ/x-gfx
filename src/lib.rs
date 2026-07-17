#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    unsafe_op_in_unsafe_fn,
    dead_code,
    unused_imports
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::items_after_statements,
    clippy::wildcard_enum_match_arm,
    clippy::inline_always,
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::wildcard_imports,
    clippy::elidable_lifetime_names,
    clippy::large_stack_arrays,
    clippy::derivable_impls,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::match_same_arms,
    clippy::new_without_default,
    clippy::missing_const_for_fn,
    clippy::struct_excessive_bools
)]

pub mod config;
pub mod frame;
pub mod palette;
pub mod surface;

#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(feature = "gpu")]
pub mod gpu;

#[cfg(feature = "mcu")]
pub mod mcu;
