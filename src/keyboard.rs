pub use windows::Win32::UI::Input::KeyboardAndMouse::*;
pub use windows::Win32::UI::WindowsAndMessaging::*;
pub use windows::Win32::Foundation::*;
use std::mem::zeroed;
use once_cell::sync::Lazy;
use std::sync::Mutex;


// ez names
pub const SHIFT: VIRTUAL_KEY = VK_LSHIFT;
pub const CONTROL: VIRTUAL_KEY = VK_LCONTROL;
pub const ALT: VIRTUAL_KEY = VK_LMENU;
pub const WINDOWS: VIRTUAL_KEY = VK_LWIN;

const BLOCK:u8 = 1;
const SWAP:u8 = 2;
const SWAP_MOD:u8 = 3;

// makes HOTKEYS more understandable
type ActionType = u8;
type KeyIsPressed = bool;
type EnableModifiers = (bool, usize);
type EnableInject = bool;
type TriggerKey = VIRTUAL_KEY;
type KeyToSend = VIRTUAL_KEY;
type ModifierKeys = VIRTUAL_KEY;

// allows use of a gobal variable 'HOTKEYS'
// pub static HOTKEYS: Lazy<Mutex<Vec<unsafe fn (n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT>>> = Lazy::new(|| Mutex::new(vec![]));
pub static HOTKEYS: Lazy<Mutex<Vec<(
    ActionType,
    KeyIsPressed,
    EnableModifiers,
    EnableInject,
    TriggerKey,
    Option<KeyToSend>,
    Option<Vec<ModifierKeys>>,
)>>> = Lazy::new(|| Mutex::new(vec![]));



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
        key_event(*self, 0, KEYBD_EVENT_FLAGS(0));
    }

    fn up(&self) {
        key_event(*self, 0, KEYEVENTF_KEYUP);
    }

    fn send(&self) {
        key_event(*self, 0, KEYBD_EVENT_FLAGS(0));
        key_event(*self, 0, KEYEVENTF_KEYUP);
    }
}

pub struct OnKeyDown;
pub struct OnKeyUp;

impl OnKeyDown {
    pub fn block(key_in: VIRTUAL_KEY) {
        HOTKEYS.lock().unwrap().push(( BLOCK, true, (false, 0), false, key_in, None, None));
    }

    pub fn swap(key_in: VIRTUAL_KEY, key_out: VIRTUAL_KEY) {
        HOTKEYS.lock().unwrap().push(( SWAP, true, (false, 0), false, key_in, Some(key_out), None));
    }

    pub fn swap(key_in: VIRTUAL_KEY, key_out: VIRTUAL_KEY) {
        HOTKEYS.lock().unwrap().push(( SWAP, true, (false, 0), false, key_in, Some(key_out), None));
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


// set hooks to monitor keyboard and mouse events
pub unsafe fn set_keyboard_hook() {

    // easy reading of 'SetWindowsHookExW' variables
    let id_hook: WINDOWS_HOOK_ID = WH_KEYBOARD_LL;
    let lpfn: HOOKPROC = Some(keyboard_hook);
    let hmod: HINSTANCE = zeroed();
    let dw_thread_id: u32 = 0;

    //  installs hook to monitor keyboard events
    let keyboard: HHOOK = SetWindowsHookExW(id_hook, lpfn, hmod, dw_thread_id);

    // message loop
    let mut message: MSG = zeroed();
    GetMessageW(&mut message, None, 0, 0);

    // not used yet
    UnhookWindowsHookEx(keyboard);
}

// each time keyboard or mouse events occur they will pass though this hook
pub unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    // window message key states  syskey = alt
    let wm_down = WPARAM(WM_KEYDOWN as usize) == w_param || WPARAM(WM_SYSKEYDOWN as usize) == w_param;
    let wm_up = WPARAM(WM_KEYUP as usize) == w_param || WPARAM(WM_SYSKEYUP as usize) == w_param;

    // data from windows hook struct
    let ll_keyboard_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as _;
    let hook_key = VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16);
    let check_inject = ((*ll_keyboard_struct).flags & LLKHF_INJECTED).0 == 0;

    let keys = HOTKEYS.lock().unwrap().clone();

    for key in keys {
    // bools
    let keys_match = key.4 == hook_key;  // current hook key and trigger key match
    let on_key_down = key.1 == true;     // preform actions when called by OnKeyDown
    let do_we_inject = key.3 == true;    // do we check if it is an injected key
    let action = key.0;                  // set current action
    // key.0 ActionType
    // key.1 KeyIsPressed
    // key.2 EnableModifiers
    // key.3 EnableInject
    // key.4 TriggerKey
    // key.5 Option<KeyToSend>
    // key.6 Option<Vec<ModifierKeys>>

    if wm_down && on_key_down && keys_match && do_we_inject && check_inject
        || wm_up && !on_key_down && keys_match && do_we_inject && check_inject
        || wm_down && on_key_down && keys_match && !do_we_inject
        || wm_up && !on_key_down && keys_match && !do_we_inject {
            match action {
                BLOCK => (),
                SWAP => key.5.unwrap().send(),
                SWAP_MOD => if key.6.unwrap()[0].is_down() {
                    key.5.unwrap().send();
                },
                _ => (),
            }
            return LRESULT(1);
        }
    }

    // no hotkey matched just return like normal
    CallNextHookEx(None, n_code, w_param, l_param)
}