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

// error 404 no key found, a fake key, no key,
const VK_FALSE: VIRTUAL_KEY = VIRTUAL_KEY(404);

// actions
const NONE:u8 = 0;
const SWAP:u8 = 2;

// allows use of a gobal variable 'HOTKEYS'
pub static HOTKEYS: Lazy<Mutex<Vec<Hotkey>>> = Lazy::new(|| Mutex::new(vec![]));

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

#[derive(Debug, Clone)]
pub struct Hotkey {
    pub trigger: VIRTUAL_KEY,
    pub modifiers: Vec<VIRTUAL_KEY>,
    pub action: u8,
    pub on_release: bool,
    pub block_input_key: bool,
    pub enable_modifiers: bool,
    pub block_inject: bool,
    pub to_swap: VIRTUAL_KEY,

}

impl Hotkey {
    pub fn new(key: VIRTUAL_KEY) -> Hotkey {
        Hotkey {
            trigger: key,
            modifiers: Vec::new(),
            action: NONE,
            on_release: false,
            block_input_key: false,
            enable_modifiers: false,
            block_inject: false,
            to_swap: VK_FALSE,
        }
    }

    pub fn add_mods(mut self, key: Vec<VIRTUAL_KEY>) -> Self {
        self.enable_modifiers = true;
        self.modifiers = key;
        self
    }

    pub fn block(mut self) -> Self {
        self.block_input_key = true;
        self
    }

    pub fn on_release(mut self) -> Self {
        self.on_release = true;
        self
    }

    pub fn block_inject(mut self) -> Self {
        self.block_inject = true;
        self
    }

    pub fn swap(mut self, key: VIRTUAL_KEY) -> Self {
        self.action = SWAP;
        self.to_swap = key;
        self
    }

    pub fn spawn(self) {
        HOTKEYS.lock().unwrap().push(self);
    }

}

// each time keyboard or mouse events occur they will pass though this hook
pub unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    // data from windows hook struct
    let ll_keyboard_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as _;

    fn modifiers_are_down(keys: Vec<VIRTUAL_KEY>) -> bool {
        for key in keys {
            if !key.is_down() { return false }
        }
        true
    }

    // bring in hotkey global variable and loop though it
    let keys = HOTKEYS.lock().unwrap().clone();
    for key in keys {
        match {
            (
                // does the current hooked key have a hotkey assigned to it?
                VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16) == key.trigger
                // and if it does!, what postion is the  hotkey assigned to trigger?
                && {  // in this case we compare if the hotkey and hooked key are being pressed
                    !key.on_release == (WPARAM(WM_KEYDOWN as usize) == w_param)
                        || WPARAM(WM_SYSKEYDOWN as usize) == w_param                
                }
                || // or this is if the hotkey triggers on an key release
                VIRTUAL_KEY((*ll_keyboard_struct).vkCode as u16) == key.trigger
                && {  // we have to compare the hotkey and hooked key again.
                    key.on_release &&
                        WPARAM(WM_KEYUP as usize) == w_param
                        || WPARAM(WM_SYSKEYUP as usize) == w_param                
                }    

            ) && ( // comparing the shit above this line to the conditions below this line

                !key.block_inject
                    || key.block_inject && ((*ll_keyboard_struct).flags & LLKHF_INJECTED).0 == 0

                && !key.enable_modifiers 
                    || key.enable_modifiers && modifiers_are_down(key.modifiers)
            )
        } {
            true => {
                match key.action {
                    SWAP => key.to_swap.send(),
                    _ => (),
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