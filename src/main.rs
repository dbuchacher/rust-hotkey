use ahk::*;

fn main() {
    
    swap(VK_A, VK_H);
    swap(VK_H, VK_D);

    block(VK_B);
    
    unsafe { set_hook(); }
}

fn block(v_key: VIRTUAL_KEY) {
    BLOCKKEYS.lock().unwrap().push(v_key);
}
fn swap(in_key: VIRTUAL_KEY, out_key: VIRTUAL_KEY) {
    SWAPKEYS.lock().unwrap().push((in_key, out_key));
}

unsafe fn test(v_key: VIRTUAL_KEY, n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let keyboard_data: *mut KBDLLHOOKSTRUCT = l_param.0 as _;
    let mouse_data: *mut MSLLHOOKSTRUCT = l_param.0 as _;

    match w_param.0 as u32 {
    WM_KEYDOWN => {
        let c: u8 = MapVirtualKeyW((*keyboard_data).vkCode, MAPVK_VK_TO_CHAR) as u8;
        println!("{}", c);
    }
    _ => (),
    }

    CallNextHookEx(None, n_code, w_param, l_param)
}

// unsafe fn test(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
//     let keyboard_data: *mut KBDLLHOOKSTRUCT = l_param.0 as _;
//     let mouse_data: *mut MSLLHOOKSTRUCT = l_param.0 as _;

//     match w_param.0 as u32 {
//     WM_KEYDOWN => {
//         let c: u8 = MapVirtualKeyW((*keyboard_data).vkCode, MAPVK_VK_TO_CHAR) as u8;
//         println!("{}", c);
//     }
//     _ => (),
//     }

//     CallNextHookEx(None, n_code, w_param, l_param)
// }








fn stop_a_key() {
    // if (*s).vkCode == 65 && VK_B.is_down() == true {
    //     println!("b + a was pressed");
    //     return LRESULT(1);        
    // }
}

// match w_param.0 as u32 {
// WM_KEYDOWN => {
//     let c: u8 = MapVirtualKeyW((*keyboard_data).vkCode, MAPVK_VK_TO_CHAR) as u8;
//     println!("{}", c);
// }
// _ => (),
// }

// WM_LBUTTONDOWN
// WM_RBUTTONDOWN
// WM_MBUTTONDOWN
// WM_XBUTTONDOWN

// let keyboard_data: *mut KBDLLHOOKSTRUCT = l_param.0 as _;
// let mouse_data: *mut MSLLHOOKSTRUCT = l_param.0 as _;

// unsafe {
//     HOTKEYS.lock().unwrap().push(
//         |n_code: i32, w_param: WPARAM, l_param: LPARAM| -> LRESULT { 
//             let keyboard_data: *mut KBDLLHOOKSTRUCT = l_param.0 as _;
//             let mouse_data: *mut MSLLHOOKSTRUCT = l_param.0 as _;
        
//             // println!("{:?}", (*mouse_data));
//             // println!("{:?}", (*keyboard_data));
        
        
//             match w_param.0 as u32 {
//                 WM_LBUTTONDOWN => {
//                 let c: u8 = MapVirtualKeyW((*keyboard_data).vkCode, MAPVK_VK_TO_CHAR) as u8;
//                 println!("{}", c);
//             }
//             _ => (),
//             }
        
        
//             if (*keyboard_data).vkCode == 65 && VK_B.is_down() == true {
//                 println!("b + a was pressed");
//                 return LRESULT(1)
//             }
//             CallNextHookEx(None, n_code, w_param, l_param)
//         }
//     );
// }

