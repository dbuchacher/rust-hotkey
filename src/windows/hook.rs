use crate::windows::*;

// couldn't find these in windows-rs
const BACK_SIDE_BUTTON:    u32 = 65536;
const FORWARD_SIDE_BUTTON: u32 = 131072;
const WHEEL_DOWN:          u32 = 4287102976;
const WHEEL_UP:            u32 = 7864320;

// each time keyboard or mouse events occur they will pass though this hook
pub unsafe extern "system" fn hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {

    // data from windows hook struct
    let ll_keyboard_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as _;
    let ll_mouse_struct: *mut MSLLHOOKSTRUCT  = l_param.0 as _;

    // keyboard hotkey global variable
    let keys = HOTKEYS.lock().unwrap().clone();

    for key in keys {

        // are the hotkey and hooked key being pressed-down
        let pressed = !key.on_release == {
            WPARAM(WM_KEYDOWN as usize) == w_param ||
            WPARAM(WM_SYSKEYDOWN as usize) == w_param ||
            WPARAM(WM_LBUTTONDOWN as usize) == w_param ||
            WPARAM(WM_RBUTTONDOWN as usize) == w_param ||
            WPARAM(WM_MBUTTONDOWN as usize) == w_param ||
            WPARAM(WM_XBUTTONDOWN as usize) == w_param ||
            WPARAM(WM_MOUSEWHEEL as usize) == w_param
        };

        // are the hotkey and hooked key being released-up
        let released = key.on_release == {
            WPARAM(WM_KEYUP as usize) == w_param ||
            WPARAM(WM_SYSKEYUP as usize) == w_param ||
            WPARAM(WM_LBUTTONUP as usize) == w_param ||
            WPARAM(WM_RBUTTONUP as usize) == w_param ||
            WPARAM(WM_MBUTTONUP as usize) == w_param ||
            WPARAM(WM_XBUTTONUP as usize) == w_param
        };

        // stops injected mouse events
        let mouse_inject = {
            !key.block_inject ||
            key.block_inject &&
            ((*ll_mouse_struct).flags & LLMHF_INJECTED) == 0
        };

        // kstops injected keyboard events
        let kbd_inject = {
            !key.block_inject ||
            key.block_inject &&
            ((*ll_keyboard_struct).flags & LLKHF_INJECTED).0 == 0
        };

        // check all modifier keys are down
        let modifiers = !key.enable_modifiers || key.enable_modifiers && modifiers_are_down(key.modifiers);

        // a whole bunch of confusing stuff
        let logic = match w_param.0 as u32 {
            WM_KEYDOWN     => (key.trigger == VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16)) && pressed  && kbd_inject   && modifiers,
            WM_SYSKEYDOWN  => (key.trigger == VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16)) && pressed  && kbd_inject   && modifiers,
            WM_LBUTTONDOWN => (key.trigger == VIRTUAL_KEY(1))                                   && pressed  && mouse_inject && modifiers,
            WM_RBUTTONDOWN => (key.trigger == VIRTUAL_KEY(2))                                   && pressed  && mouse_inject && modifiers,
            WM_MBUTTONDOWN => (key.trigger == VIRTUAL_KEY(4))                                   && pressed  && mouse_inject && modifiers,
            WM_KEYUP       => (key.trigger == VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16)) && released && kbd_inject   && modifiers,
            WM_SYSKEYUP    => (key.trigger == VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16)) && released && kbd_inject   && modifiers,
            WM_LBUTTONUP   => (key.trigger == VIRTUAL_KEY(1))                                   && released && mouse_inject && modifiers,
            WM_RBUTTONUP   => (key.trigger == VIRTUAL_KEY(2))                                   && released && mouse_inject && modifiers,
            WM_MBUTTONUP   => (key.trigger == VIRTUAL_KEY(4))                                   && released && mouse_inject && modifiers,
            WM_XBUTTONDOWN => match (*ll_mouse_struct).mouseData {
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(BACK_SIDE_BUTTON)    => (key.trigger == VIRTUAL_KEY(5)) && pressed && mouse_inject && modifiers,
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(FORWARD_SIDE_BUTTON) => (key.trigger == VIRTUAL_KEY(6)) && pressed && mouse_inject && modifiers,
                _ => continue,
            },
            WM_XBUTTONUP => match (*ll_mouse_struct).mouseData {
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(BACK_SIDE_BUTTON)    => (key.trigger == VIRTUAL_KEY(5)) && released && mouse_inject && modifiers,
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(FORWARD_SIDE_BUTTON) => (key.trigger == VIRTUAL_KEY(6)) && released && mouse_inject && modifiers,
                _ => continue,
            },
            WM_MOUSEWHEEL => match (*ll_mouse_struct).mouseData {
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(WHEEL_DOWN)         => (key.trigger == VIRTUAL_KEY(300)) && pressed && mouse_inject && modifiers,
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(WHEEL_UP)           => (key.trigger == VIRTUAL_KEY(301)) && pressed && mouse_inject && modifiers,
                _ => continue,
            },
            _ => false,
        };

        // if the confusing stuff is true we can apply the proper actions to the hotkey
        if logic == true {
            match key.action {
                HotkeyActions::None => (),
                HotkeyActions::Code => key.code.unwrap()(),
                HotkeyActions::Swap => key.to_swap.unwrap().send(),
            }
            // do we send the intial trigger key as a button push? or do we block it by returning early
            if key.block_input_key { 
                return LRESULT(1)
            }
        }        
    }
    
    // no hotkey call next hook
    CallNextHookEx(None, n_code, w_param, l_param)
}

// check if the user is holding down the mod keys
fn modifiers_are_down(keys: Vec<VIRTUAL_KEY>) -> bool {
    for key in keys {
        if !key.is_down() { return false }
    }
    true
}