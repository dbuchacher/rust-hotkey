use crate::windows::*;

// each time keyboard or mouse events occur they will pass though this hook
pub unsafe extern "system" fn hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    // data from windows hook struct
    let ll_keyboard_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as _;

    // keyboard hotkey global variable
    let keys = HOTKEYS.lock().unwrap().clone();

    for key in keys {
        // does the current hooked key have a hotkey assigned to it?
        let keys_match = VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16) == key.trigger;

        // are the hotkey and hooked key a keyboard key being pressed-down
        let keyboard_pressed = !key.on_release == {
            WPARAM(WM_KEYDOWN as usize) == w_param ||
            WPARAM(WM_SYSKEYDOWN as usize) == w_param
        };

        // are the hotkey and hooked key a keyboard key being released-up
        let keyboard_released = key.on_release == {
            WPARAM(WM_KEYUP as usize) == w_param ||
            WPARAM(WM_SYSKEYUP as usize) == w_param
        };

        // keyboard inject
        let kbd_inject = {
            !key.block_inject ||
            key.block_inject &&
            ((*ll_keyboard_struct).flags & LLKHF_INJECTED).0 == 0
        };

        // modifiers
        let modifiers = !key.enable_modifiers || key.enable_modifiers && modifiers_are_down(key.modifiers);

        // combine conditions
        let pressed_logic = keys_match && keyboard_pressed  && kbd_inject && modifiers;
        let released_logic = keys_match && keyboard_released  && kbd_inject && modifiers;

        match pressed_logic || released_logic {
            true => {
                match key.action {
                    HotkeyActions::None => (),
                    HotkeyActions::Code => key.code.unwrap()(),
                    HotkeyActions::Swap => match pressed_logic {
                        true => key.to_swap.unwrap().down(), // when key is being pressed down
                        false => key.to_swap.unwrap().up(),  // when key is being released up
                    },
                }
                // do we send the intial trigger key as a button push? or do we block it?
                match key.block_input_key {
                    true => return LRESULT(1),
                    false => return CallNextHookEx(None, n_code, w_param, l_param),
                }
            },
            // no hotkey matched lets continue trying the next
            false => continue,
        }
        
    }
    
    CallNextHookEx(None, n_code, w_param, l_param)
}

fn modifiers_are_down(keys: Vec<VIRTUAL_KEY>) -> bool {
    for key in keys {
        if !key.is_down() { return false }
    }
    true
}