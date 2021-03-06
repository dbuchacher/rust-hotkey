pub use windows::Win32::UI::Input::KeyboardAndMouse::*;
pub use crate::KeyState::{ OnUp, OnDown };

use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::*;
use std::mem::zeroed;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub mod hook;

// enables the mouse wheel to be used like the rest of the keys
pub const VK_WHEELDOWN: VIRTUAL_KEY = VIRTUAL_KEY(300u16);
pub const VK_WHEELUP: VIRTUAL_KEY = VIRTUAL_KEY(301u16);

// adds functionality to VIRTUAL_KEY types
pub trait Actions {
    fn is_up(&self) -> bool;     // VK_A.is_down()
    fn is_down(&self) -> bool;   // VK_A.is_up()
    fn is_toggle_on(&self) -> bool; // VK_A.toggle_on
    fn down(&self);              // VK_A.down()
    fn up(&self);                // VK_A.up()
    fn send(&self);              // VK_A.send()
}

impl Actions for VIRTUAL_KEY {
    // return true if key is down
    fn is_down(&self) -> bool {
        unsafe { GetAsyncKeyState(self.0 as i32) as u16 & 0x8000 != 0 }
    }
    // return true if key is up
    fn is_up(&self) -> bool {
        unsafe { GetAsyncKeyState(self.0 as i32) as u16 & 0x8000 == 0 }
    }
    fn is_toggle_on(&self) -> bool {
        (unsafe { GetKeyState(self.0 as i32) } != 0)
    }
    // push the key down
    fn down(&self) {
        press_logic(*self);
    }
    // release the key up
    fn up(&self) {
        release_logic(*self);
    }
    // push the key down then release the key up
    fn send(&self) {
        press_logic(*self);
        release_logic(*self);
    }
}

// send logic: reduces duplicate code
fn press_logic(key: VIRTUAL_KEY) {
    match key {
        VK_LBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_LEFTDOWN),
        VK_RBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_RIGHTDOWN),
        VK_MBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_MIDDLEDOWN),
        VK_XBUTTON1 => mouse_event(XBUTTON1, MOUSEEVENTF_XDOWN),
        VK_XBUTTON2 => mouse_event(XBUTTON2, MOUSEEVENTF_XDOWN),
        VK_WHEELDOWN => mouse_event(MOUSEHOOKSTRUCTEX_MOUSE_DATA(u32::MAX - WHEEL_DELTA/2), MOUSEEVENTF_WHEEL),
        VK_WHEELUP => mouse_event(MOUSEHOOKSTRUCTEX_MOUSE_DATA(WHEEL_DELTA/2), MOUSEEVENTF_WHEEL),
        _ => key_event(key, 0, KEYBD_EVENT_FLAGS(0))
    }
}
fn release_logic(key: VIRTUAL_KEY) {
    match key {
        VK_LBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_LEFTUP),
        VK_RBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_RIGHTUP),
        VK_MBUTTON => mouse_event(unsafe { zeroed() }, MOUSEEVENTF_MIDDLEUP),
        VK_XBUTTON1 => mouse_event(XBUTTON1, MOUSEEVENTF_XUP),
        VK_XBUTTON2 => mouse_event(XBUTTON2, MOUSEEVENTF_XUP),
        VK_WHEELDOWN => mouse_event(MOUSEHOOKSTRUCTEX_MOUSE_DATA(u32::MAX - WHEEL_DELTA/2), MOUSEEVENTF_WHEEL),
        VK_WHEELUP => mouse_event(MOUSEHOOKSTRUCTEX_MOUSE_DATA(WHEEL_DELTA/2), MOUSEEVENTF_WHEEL),
        _ => key_event(key, 0, KEYEVENTF_KEYUP)
    }
}

// sends a key event
fn key_event(v_key: VIRTUAL_KEY, w_scan: u16, dw_flags: KEYBD_EVENT_FLAGS) {

    // 'SendInput' variables
    let c_inputs = 1;
    let p_inputs = [ 
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
    ];
    let c_bsize = std::mem::size_of::<INPUT>() as i32;

    // call windows api to do the magic
    unsafe { SendInput(c_inputs, &p_inputs[0], c_bsize); }
}

// sends a mouse event
fn mouse_event(mouse_data: MOUSEHOOKSTRUCTEX_MOUSE_DATA, dw_flags: MOUSE_EVENT_FLAGS) {

    // 'SendInput' variables
    let c_inputs = 1;
    let p_inputs = [ 
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
    ];
    let c_bsize = std::mem::size_of::<INPUT>() as i32;

    // call windows api to do the magic
    unsafe { SendInput(c_inputs, &p_inputs[0], c_bsize); }
}



// allows access of gobal variable 'HOTKEYS' from when we are in the event hook
pub static HOTKEYS: Lazy<Mutex<Vec<Hotkey>>> = Lazy::new(|| Mutex::new(vec![]));

// things that can happen when a hotkey is activated
#[derive(Debug, Clone)]
pub enum HotkeyActions {
    None,
    Send,
    Code,
    Down,
    Up,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyState {
    OnDown,
    OnUp,
}

// different type of options a hotkey can contain
#[derive(Debug, Clone)]
pub struct Hotkey {
    pub trigger: VIRTUAL_KEY,
    pub modifiers: Vec<VIRTUAL_KEY>,
    pub action: HotkeyActions,
    pub state: Option<KeyState>,
    pub unblock: bool,
    pub enable_modifiers: bool,
    pub block_inject: bool,
    pub code: Option<fn ()>,
    pub send: Option<VIRTUAL_KEY>,
    pub down: Option<VIRTUAL_KEY>,
    pub up: Option<VIRTUAL_KEY>,
}

impl Hotkey {
    // default values for new hotkeys
    pub fn new(key: VIRTUAL_KEY) -> Hotkey {
        Hotkey {
            trigger: key,
            modifiers: Vec::new(),
            action: HotkeyActions::None,
            state: None,
            unblock: false,
            enable_modifiers: false,
            block_inject: false,
            code: None,
            send: None,
            down: None,
            up: None,
        }
    }
    // without spawn the hotkey won't be active
    pub fn spawn(mut self, postion: KeyState) {
        // select if hotkey is trigger on a key press or key release
        self.state = Some(postion);
        HOTKEYS.lock().unwrap().push(self);
    }
    // delete a hotkey
    pub fn remove(key: VIRTUAL_KEY) {
        HOTKEYS.lock().unwrap().retain(|x| x.trigger != key);
    }
    // delete all hotkeys
    pub fn remove_all() {
        HOTKEYS.lock().unwrap().clear();
    }
    // add modifier keys that need to be pressed before the hotkey is pressed
    pub fn mods(mut self, key: Vec<VIRTUAL_KEY>) -> Self {
        self.enable_modifiers = true;
        self.modifiers = key;
        self
    }
    // unblock the hotkey key from also being sent
    pub fn unblock(mut self) -> Self {
        self.unblock = true;
        self
    }
    // block keys that have been injected
    pub fn block_inject(mut self) -> Self {
        self.block_inject = true;
        self
    }
    // run code from an external function
    pub fn code(mut self, code: fn ()) -> Self {
        self.action = HotkeyActions::Code;
        self.code = Some(code);
        self
    }
    // key event down than up
    pub fn send(mut self, key: VIRTUAL_KEY) -> Self {
        self.action = HotkeyActions::Send;
        self.send = Some(key);
        self
    }
    // key event down
    pub fn down(mut self, key: VIRTUAL_KEY) -> Self {
        self.action = HotkeyActions::Down;
        self.down = Some(key);
        self
    }
    // key event up
    pub fn up(mut self, key: VIRTUAL_KEY) -> Self {
        self.action = HotkeyActions::Up;
        self.up = Some(key);
        self
    }
}




// // returns x/y coords of the mouse cursor
// pub fn postion_get() -> (i32, i32) {
//     let mut lppoint = windows::Win32::Foundation::POINT { x: 0, y: 0 };

//     unsafe { GetCursorPos(&mut lppoint); }

//     (lppoint.x, lppoint.y)
// }

// // sets x/y coords of the mouse cursor
// pub fn postion_set(x: i32, y: i32) {
//     unsafe { SetCursorPos(x, y); }
// }