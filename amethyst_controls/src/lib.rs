//! Amethyst control crate.

#![warn(missing_docs, rust_2018_idioms, rust_2018_compatibility)]
// I have to comment this out as right now no version of this doesn't generate warnings on both stable and beta.
// #![allow(clippy::type_complexity)] // complex project

extern crate amethyst_assets;
extern crate amethyst_core;
extern crate amethyst_input;
extern crate amethyst_renderer;
#[macro_use]
extern crate serde;
extern crate winit;

#[cfg(feature = "profiler")]
extern crate thread_profiler;

mod bundles;
mod components;
mod resources;
mod systems;

pub use self::{
    bundles::{ArcBallControlBundle, FlyControlBundle},
    components::{ArcBallControlTag, ControlTagPrefab, FlyControlTag},
    resources::{HideCursor, WindowFocus},
    systems::{
        ArcBallRotationSystem, CursorHideSystem, FlyMovementSystem, FreeRotationSystem,
        MouseFocusUpdateSystem,
    },
};
