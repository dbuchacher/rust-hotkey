
// fn down(data: &HookData, trigger_key: VIRTUAL_KEY) -> bool {

//     match data.w_param.0 as u32 {
//         WM_KEYDOWN => if trigger_key == data.hooked_key { return true },
//         WM_LBUTTONDOWN => if trigger_key == VIRTUAL_KEY(1) { return true },
//         WM_RBUTTONDOWN => if trigger_key == VIRTUAL_KEY(2) { return true },
//         WM_MBUTTONDOWN => if trigger_key == VIRTUAL_KEY(4) { return true },
//         WM_XBUTTONDOWN => match data.mouse_data {
//             MOUSEHOOKSTRUCTEX_MOUSE_DATA(65536) => if trigger_key == VIRTUAL_KEY(5) { return true },
//             MOUSEHOOKSTRUCTEX_MOUSE_DATA(131072) => if trigger_key == VIRTUAL_KEY(6) { return true },
//             _ => (),
//         },
//         WM_MOUSEWHEEL => match data.mouse_data {
//             MOUSEHOOKSTRUCTEX_MOUSE_DATA(4287102976) => if trigger_key == VIRTUAL_KEY(300) { return true },
//             MOUSEHOOKSTRUCTEX_MOUSE_DATA(7864320) => if trigger_key == VIRTUAL_KEY(300) { return true },
//             _ => (),
//         },
//         _ => (),
//     }
//     return false
// }

const BACK_SIDE_BUTTON: u32 = 65536;
const FORWARD_SIDE_BUTTON: u32 = 131072;

use crate::windows::*;

// each time keyboard or mouse events occur they will pass though this hook
pub unsafe extern "system" fn hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    // data from windows hook struct
    let ll_mouse_struct: *mut MSLLHOOKSTRUCT  = l_param.0 as _;

    // keyboard hotkey global variable
    let keys = HOTKEYS.lock().unwrap().clone();

    for key in keys {

                // are the hotkey and hooked key a keyboard key being pressed-down
                let pressed = !key.on_release == {
                    WPARAM(WM_LBUTTONDOWN as usize) == w_param ||
                    WPARAM(WM_RBUTTONDOWN as usize) == w_param ||
                    WPARAM(WM_MBUTTONDOWN as usize) == w_param ||
                    WPARAM(WM_XBUTTONDOWN as usize) == w_param
                };
        
                // are the hotkey and hooked key a keyboard key being released-up
                let released = key.on_release == {
                    WPARAM(WM_LBUTTONUP as usize) == w_param ||
                    WPARAM(WM_RBUTTONUP as usize) == w_param ||
                    WPARAM(WM_MBUTTONUP as usize) == w_param ||
                    WPARAM(WM_XBUTTONUP as usize) == w_param
                };
        
                // mouse inject condition
                let mouse_inject = {
                    !key.block_inject ||
                    key.block_inject &&
                    ((*ll_mouse_struct).flags & LLMHF_INJECTED) == 0
                };
        
                // modifiers
                let modifiers = !key.enable_modifiers || key.enable_modifiers && modifiers_are_down(key.modifiers);


        let shit = match w_param.0 as u32 {
            WM_LBUTTONDOWN => (key.trigger == VIRTUAL_KEY(1)) && pressed && mouse_inject && modifiers,
            WM_RBUTTONDOWN => (key.trigger == VIRTUAL_KEY(2)) && pressed && mouse_inject && modifiers,
            WM_MBUTTONDOWN => (key.trigger == VIRTUAL_KEY(4)) && pressed && mouse_inject && modifiers,
            WM_LBUTTONUP   => (key.trigger == VIRTUAL_KEY(1)) && released  && mouse_inject && modifiers,
            WM_RBUTTONUP   => (key.trigger == VIRTUAL_KEY(2)) && released  && mouse_inject && modifiers,
            WM_MBUTTONUP   => (key.trigger == VIRTUAL_KEY(4)) && released  && mouse_inject && modifiers,
            WM_XBUTTONDOWN => match (*ll_mouse_struct).mouseData {
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(BACK_SIDE_BUTTON) => (key.trigger == VIRTUAL_KEY(5)) && pressed && mouse_inject && modifiers,
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(FORWARD_SIDE_BUTTON) => (key.trigger == VIRTUAL_KEY(6)) && pressed && mouse_inject && modifiers,
                _ => continue,
            },
            WM_XBUTTONUP => match (*ll_mouse_struct).mouseData {
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(BACK_SIDE_BUTTON) => (key.trigger == VIRTUAL_KEY(5)) && released  && mouse_inject && modifiers,
                MOUSEHOOKSTRUCTEX_MOUSE_DATA(FORWARD_SIDE_BUTTON) => (key.trigger == VIRTUAL_KEY(6)) && released  && mouse_inject && modifiers,
                _ => continue,
            },
            _ => false,
        };


        // let kkk = match shit {
        //     WM_LBUTTONDOWN => pressed && mouse_inject && modifiers,
        //     WM_RBUTTONDOWN => pressed && mouse_inject && modifiers,
        //     WM_MBUTTONDOWN => pressed && mouse_inject && modifiers,
        //     WM_LBUTTONUP => released  && mouse_inject && modifiers,
        //     WM_RBUTTONUP => released  && mouse_inject && modifiers,
        //     WM_MBUTTONUP => released  && mouse_inject && modifiers,
        //     _ => false,
        // };

        // let pressed_logic = kkk && pressed  && mouse_inject && modifiers;
        // let released_logic = kkk && released  && mouse_inject && modifiers;

        match shit {
            true => {
                match key.action {
                    HotkeyActions::None => (),
                    HotkeyActions::Code => key.code.unwrap()(),
                    HotkeyActions::Swap => key.to_swap.unwrap().send(), // match shit {
                    //     true => key.to_swap.unwrap().down(), // when key is being pressed down
                    //     false => key.to_swap.unwrap().up(),  // when key is being released up
                    // },
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
        

        // println!("{:?} \n {:?}", w_param.0, (*ll_mouse_struct).mouseData);
        // println!("{:?}", keys_match);

    }
    
    CallNextHookEx(None, n_code, w_param, l_param)
}

fn modifiers_are_down(keys: Vec<VIRTUAL_KEY>) -> bool {
    for key in keys {
        if !key.is_down() { return false }
    }
    true
}