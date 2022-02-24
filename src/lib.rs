#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use crate::windows::*;
pub use crate::windows::keyboard_hook::*;
pub use crate::windows::mouse_hook::*;