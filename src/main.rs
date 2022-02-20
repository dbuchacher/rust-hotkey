use ahk::*;
pub use windows::Win32::UI::Input::KeyboardAndMouse::*;


fn main() {

    OnKeyDown::block(VK_A);
    OnKeyDown::swap(VK_B, VK_0);
    OnKeyDown::swap_mod(VK_D, VK_E, 1);


    unsafe { set_keyboard_hook(); }
}