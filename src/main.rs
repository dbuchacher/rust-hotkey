use ahk::*;


fn mine() {
    
    println!("Rbutton is down = {}", VK_RBUTTON.is_down());
    VK_A.send();
    
}

fn main() {

    println!("{:?}", WM_XBUTTONDOWN);

    Hotkey::new(VK_XBUTTON2)
        .swap(VK_M)
        .block()
        .spawn();

    Hotkey::new(VK_RBUTTON)
        .swap(VK_R)
        .block()
        .spawn();

    Hotkey::new(VK_MBUTTON)
        .swap(VK_M)
        .on_release()
        .spawn();

    Hotkey::new(VK_A)
        .code(mine)
        .block_inject()
        .block()
        .spawn();

    set_hook();


    
}

// (
//     // does the current hooked key have a hotkey assigned to it?
//     VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16) == key.trigger
//     // and if it does!, what key-postion is the hotkey assigned to trigger?
//     && {  // in this case we compare if the hotkey and hooked key are being pressed-down
//         !key.on_release == (WPARAM(WM_KEYDOWN as usize) == w_param)
//             || WPARAM(WM_SYSKEYDOWN as usize) == w_param                
//     }
//     || // or this is when the hotkey triggers on a key release-up
//     VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16) == key.trigger
//     && {  // we have to compare the hotkey and hooked key postion again.
//         key.on_release &&
//             WPARAM(WM_KEYUP as usize) == w_param
//             || WPARAM(WM_SYSKEYUP as usize) == w_param                
//     }    

// ) && ( // below this line we are checking some additional conditions

//     !key.block_inject
//         || key.block_inject && ((*ll_keyboard_struct).flags & LLKHF_INJECTED).0 == 0

//     && !key.enable_modifiers 
//         || key.enable_modifiers && modifiers_are_down(key.modifiers)
// )

        // // mouse inject condition
        // let mouse_inject = {
        //     !key.block_inject ||
        //     key.block_inject &&
        //     ((*ll_mouse_struct).flags & LLMHF_INJECTED) == 0
        // };



    //     WM_XBUTTONDOWN => match (*ll_mouse_struct).mouseData {
    //         MOUSEHOOKSTRUCTEX_MOUSE_DATA(65536) => if key.trigger == VIRTUAL_KEY(5) { 65536 } else { 0 },
    //         MOUSEHOOKSTRUCTEX_MOUSE_DATA(131072) => if key.trigger == VIRTUAL_KEY(6) { 131072 } else { 0 },
    //         _ => continue,
    //     },
    //     WM_MOUSEWHEEL => match (*ll_mouse_struct).mouseData {
    //         MOUSEHOOKSTRUCTEX_MOUSE_DATA(4287102976) => if key.trigger == VIRTUAL_KEY(300) { 4287102976 }  else { 0 },
    //         MOUSEHOOKSTRUCTEX_MOUSE_DATA(7864320) => if key.trigger == VIRTUAL_KEY(300) { 7864320 } else { 0 },
    //         _ => continue,
    //     },
    //     _ => continue,
    // };

    // let fuck = match w_param.0 as u32 {
    //     WM_LBUTTONUP => if key.trigger == VIRTUAL_KEY(1) { WM_LBUTTONUP } else { 0 },
    //     WM_RBUTTONUP => if key.trigger == VIRTUAL_KEY(2) { WM_RBUTTONUP } else { 0 },
    //     WM_MBUTTONUP => if key.trigger == VIRTUAL_KEY(4) { WM_MBUTTONUP } else { 0 },
    //     WM_XBUTTONUP => match (*ll_mouse_struct).mouseData {
    //         MOUSEHOOKSTRUCTEX_MOUSE_DATA(65536) => if key.trigger == VIRTUAL_KEY(5) { 65536 } else { 0 },
    //         MOUSEHOOKSTRUCTEX_MOUSE_DATA(131072) => if key.trigger == VIRTUAL_KEY(6) { 131072 } else { 0 },
    //         _ => continue,
    //     },
    //     _ => continue,
    // };

    // WM_XBUTTONDOWN => match (*ll_mouse_struct).mouseData {
    //     MOUSEHOOKSTRUCTEX_MOUSE_DATA(65536) => if key.trigger == VIRTUAL_KEY(5) {  65536 } else { 0 },
    //     MOUSEHOOKSTRUCTEX_MOUSE_DATA(131072) => if key.trigger == VIRTUAL_KEY(6) {  131072 } else { 0 },
    //     _ => continue,
    // },
    // WM_XBUTTONUP => match (*ll_mouse_struct).mouseData {
    //     MOUSEHOOKSTRUCTEX_MOUSE_DATA(65536) => if key.trigger == VIRTUAL_KEY(5) {  65536 } else { 0 },
    //     MOUSEHOOKSTRUCTEX_MOUSE_DATA(131072) => if key.trigger == VIRTUAL_KEY(6) {  131072 } else { 0 },
    //     _ => continue,
    // },
    // _ => continue,