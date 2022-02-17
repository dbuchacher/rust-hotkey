pub use windows::Win32::UI::Input::KeyboardAndMouse::*;
pub use windows::Win32::UI::WindowsAndMessaging::*;
pub use windows::Win32::Foundation::*;
pub use std::mem::zeroed;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// adds functionality to VIRTUAL_KEY types
    pub trait Actions {
        fn is_up(&self) -> bool;    // VK_A.is_down()
        fn is_down(&self) -> bool;  // VK_A.is_up()
        fn down(&self);             // VK_A.down()
        fn up(&self);               // VK_A.up()
        fn send(&self);             // VK_A.send()
    }

    impl Actions for VIRTUAL_KEY {
        fn is_down(&self) -> bool {
            unsafe { GetAsyncKeyState(self.0 as i32) as u16 & 0x8000 != 0 }
        }

        fn is_up(&self) -> bool {
            unsafe { GetAsyncKeyState(self.0 as i32) as u16 & 0x8000 == 0 }
        }

        fn down(&self) {
            send_down_logic(*self);
        }

        fn up(&self) {
            send_up_logic(*self);
        }

        fn send(&self) {
            send_down_logic(*self);
            send_up_logic(*self);
        }
    }


// send logic: reduces duplicate code
    fn send_down_logic(v_key: VIRTUAL_KEY) {
        match v_key {
            VK_LBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_LEFTDOWN),
            VK_RBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_RIGHTDOWN),
            VK_MBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_MIDDLEDOWN),
            VK_XBUTTON1 => mouse_event(XBUTTON1, MOUSEEVENTF_XDOWN),
            VK_XBUTTON2 => mouse_event(XBUTTON2, MOUSEEVENTF_XDOWN),
            _ => key_event(v_key, 0, KEYBD_EVENT_FLAGS(0))
        }
    }

    fn send_up_logic(v_key: VIRTUAL_KEY) {
        match v_key {
            VK_LBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_LEFTUP),
            VK_RBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_RIGHTUP),
            VK_MBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_MIDDLEUP),
            VK_XBUTTON1 => mouse_event(XBUTTON1, MOUSEEVENTF_XUP),
            VK_XBUTTON2 => mouse_event(XBUTTON2, MOUSEEVENTF_XUP),
            _ => key_event(v_key, 0, KEYEVENTF_KEYUP)
        }
    }


// sends a key event
fn key_event(v_key: VIRTUAL_KEY, w_scan: u16, dw_flags: KEYBD_EVENT_FLAGS) {

    // 'SendInput' variables
    let c_inputs = 1;
    let mut p_inputs = { 
        INPUT {
            r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: v_key,        // zero if (dw_flags == KEYEVENTF_UNICODE) else VIRTUAL_KEY
                        wScan: w_scan,     // if (dw_flags == KEYEVENTF_UNICODE) a unicode character is sent
                        dwFlags: dw_flags, // 0 = down / KEYEVENTF_KEYUP / KEYEVENTF_UNICODE
                        time: 0,
                        dwExtraInfo: 0,
                    }
                }
        }
    };
    let c_bsize = std::mem::size_of::<INPUT>() as i32;

    // call windows api to do the magic
    unsafe { SendInput(c_inputs, &mut p_inputs, c_bsize); }
}

// sends a mouse event
pub fn mouse_event(mouse_data: MOUSEHOOKSTRUCTEX_MOUSE_DATA, dw_flags: MOUSE_EVENT_FLAGS) {

    // 'SendInput' variables
    let c_inputs = 1;
    let mut p_inputs = { 
        INPUT {
            r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: mouse_data.0, // usualy zero | MOUSEEVENTF_WHEEL | MOUSEEVENTF_XDOWN | MOUSEEVENTF_XUP
                        dwFlags: dw_flags,       // MOUSEEVENTF_(LEFT,RIGHT,MIDDLE,X,)(DOWN OR UP)
                        time: 0,
                        dwExtraInfo: 0, 
                    }
                }
        }
    };
    let c_bsize = std::mem::size_of::<INPUT>() as i32;

    // call windows api to do the magic
    unsafe { SendInput(c_inputs, &mut p_inputs, c_bsize); }
}

/* later stuff maybe

    && ((*keyboard_data).flags & LLKHF_INJECTED).0 == 0
    will act like '$' modifier in auto hotkey
    
    without LLKHF_INJECTEED
    WM_KEYDOWN => if swapped_key.0.0 == v_key {}
        swap(VK_A, VK_B);
        swap(vk_B, VK_C);
            will send 'c' when you press 'a'

    with LLKHF_INJECTEED
    WM_KEYDOWN => if swapped_key.0.0 == v_key && ((*keyboard_data).flags & LLKHF_INJECTED).0 == 0 {}
        swap(VK_A, VK_B);
        swap(vk_B, VK_C);
            will send 'b' when you press 'a'
*/

// each time keyboard or mouse events occur they will pass though this hook
pub unsafe extern "system" fn hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    // windows data variables storing info about current flags
    let keyboard_data: *mut KBDLLHOOKSTRUCT = l_param.0 as _;
    let mouse_data: *mut MSLLHOOKSTRUCT = l_param.0 as _;
    let v_key = MapVirtualKeyW((*keyboard_data).vkCode, MAPVK_VK_TO_CHAR) as u16;

    // clone global variables to prevent freeze up when looping them
    let swapped_keys = SWAPKEYS.lock().unwrap().clone();
    let blocked_keys = BLOCKKEYS.lock().unwrap().clone();

    // for hotkey in HOTKEYS.lock().unwrap().iter() {
    //     // check hotkey
    //     let early_return = hotkey(n_code, w_param, l_param);

    //     // if a hotkey condition is met we will return early
    //     if early_return != CallNextHookEx(None, n_code, w_param, l_param) {
    //         return early_return
    //     }
    // }

    for swapped_key in swapped_keys {
        match w_param.0 as u32 {
            WM_KEYDOWN => if swapped_key.0.0 == v_key && ((*keyboard_data).flags & LLKHF_INJECTED).0 == 0 { 
                swapped_key.1.send(); return LRESULT(1)
            },
            _ => (),
        }
    }
    
    for blocked_key in blocked_keys {
        match w_param.0 as u32 {
            WM_KEYDOWN     => if blocked_key.0 == v_key        { return LRESULT(1) },
            WM_KEYUP       => if blocked_key.0 == v_key        { return LRESULT(1) },
            WM_LBUTTONDOWN => if blocked_key == VIRTUAL_KEY(1) { return LRESULT(1) },
            WM_LBUTTONUP   => if blocked_key == VIRTUAL_KEY(1) { return LRESULT(1) },
            WM_RBUTTONDOWN => if blocked_key == VIRTUAL_KEY(2) { return LRESULT(1) },
            WM_RBUTTONUP   => if blocked_key == VIRTUAL_KEY(2) { return LRESULT(1) },
            WM_MBUTTONDOWN => if blocked_key == VIRTUAL_KEY(4) { return LRESULT(1) },
            WM_MBUTTONUP   => if blocked_key == VIRTUAL_KEY(4) { return LRESULT(1) },
            WM_XBUTTONDOWN => if blocked_key == VIRTUAL_KEY(5) { return LRESULT(1) },
            WM_XBUTTONUP   => if blocked_key == VIRTUAL_KEY(5) { return LRESULT(1) },
            _ => (),
        }
    }

    CallNextHookEx(None, n_code, w_param, l_param)
}

// set hooks to monitor keyboard and mouse events
pub unsafe fn set_hook() {

    // easy reading of 'SetWindowsHookExW' variables
    let id_hook_keyboard: WINDOWS_HOOK_ID = WH_KEYBOARD_LL;
    let id_hook_mouse: WINDOWS_HOOK_ID = WH_MOUSE_LL;
    let lpfn: HOOKPROC = Some(hook);
    let hmod: HINSTANCE = zeroed();
    let dw_thread_id: u32 = 0;

    //  installs hook to monitor keyboard events
    let keyboard: HHOOK = SetWindowsHookExW(id_hook_keyboard, lpfn, hmod, dw_thread_id);
    let mouse: HHOOK = SetWindowsHookExW(id_hook_mouse, lpfn, hmod, dw_thread_id);

    // message loop
    let mut message: MSG = zeroed();
    GetMessageW(&mut message, None, 0, 0);

    // not used yet
    UnhookWindowsHookEx(keyboard);
    UnhookWindowsHookEx(mouse);
}


// allows use of a gobal variable 'HOTKEYS' (a vector of functions)
// using this to create different functions for each hotkey
pub static HOTKEYS: Lazy<Mutex<Vec<unsafe fn (n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT>>> = Lazy::new(|| Mutex::new(vec![]));

// block keys
pub static BLOCKKEYS: Lazy<Mutex<Vec<VIRTUAL_KEY>>> = Lazy::new(|| Mutex::new(vec![]));

// swap keys
pub static SWAPKEYS: Lazy<Mutex<Vec<(VIRTUAL_KEY, VIRTUAL_KEY)>>> = Lazy::new(|| Mutex::new(vec![]));

// use add_hotkey in 'main' to add a functions to 'HOTKEYS'
// these will be checked in the hook message loop
pub fn add_hotkey(hotkey_function: unsafe fn(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT) {
    HOTKEYS.lock().unwrap().push(hotkey_function);
}