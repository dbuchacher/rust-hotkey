use ahk::*;
pub use windows::Win32::UI::Input::KeyboardAndMouse::*;

fn main() {


    Hotkey::new(VK_A).swap(VK_0).block().spawn();
    Hotkey::new(VK_0).swap(VK_1).block().spawn();
    Hotkey::new(VK_1).swap(VK_2).block_inject().spawn();
    Hotkey::new(VK_O).block().spawn();

    Hotkey::new(VK_A)
        .add_mods(vec![VK_S, VK_D, VK_F])
        .swap(VK_0)
        .spawn();

    unsafe { set_keyboard_hook(); }

    


}