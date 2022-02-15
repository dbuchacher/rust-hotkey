use ahk::*;

fn main() {
    // HOTKEYS.lock().unwrap().push(a);

    add_hotkey(a);

    unsafe { set_hook(); }
}

fn stop_a_key() {

    // if (*s).vkCode == 65 && VK_B.is_down() == true {
    //     println!("b + a was pressed");
    //     return LRESULT(1);        
    // }

}

fn a(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        let keyboard_data: *mut KBDLLHOOKSTRUCT = l_param.0 as _;
        // let mouse_data: *mut MSLLHOOKSTRUCT = l_param.0 as _;

        if (*keyboard_data).vkCode == 65 && VK_B.is_down() == true {
            println!("b + a was pressed");
            return LRESULT(1)
        }

        CallNextHookEx(None, n_code, w_param, l_param)
    }


  //  println!("a {}", (*keyboard_data).vkCode );
}


fn b() {
    println!("b");
}