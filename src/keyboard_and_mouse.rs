// pub use windows::Win32::UI::Input::KeyboardAndMouse::*;
// pub use windows::Win32::UI::WindowsAndMessaging::*;

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

// // checks if a VK_???? key is up or down
// pub fn key_state(key: u16) -> bool {
//     unsafe { GetAsyncKeyState(key as i32) as u16 & 0x8000 != 0 }
// }
// // send a unicode string of chars
// pub fn send_chars(s: &str) {
//     for c in s.chars() {
//         key_event(EventType::KEYBDINPUT, 0, KEYEVENTF_UNICODE, c)
//     }
// }

// // send a single keyboard or mouse input down
// pub fn send_down(key: u16) {
//     send_logic(key, 0);
// }

// // send a single keyboard or mouse input up
// pub fn send_up(key: u16) {
//     send_logic(key, 2);
// }

// // send a single keyboard or mouse input down then up
// pub fn send_key(key: u16) {
//     send_logic(key, 0);
//     send_logic(key, 2);
// }

// // send logic: reduces duplicate code
// fn send_logic(key: u16, down_or_up: u32) {
//     if key <= 0 || key >= 256 {
//         panic!("send_ key is out of range");
//     }
//     if down_or_up == 0 { // down
//         match key {
//             1 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_LEFTDOWN, 0, 0 as char),
//             2 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_RIGHTDOWN, 0, 0 as char),
//             4 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_MIDDLEDOWN, 0, 0 as char),
//             5 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_XDOWN, XBUTTON1, 0 as char),
//             6 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_XDOWN, XBUTTON2, 0 as char),
//             _ => key_event(EventType::KEYBDINPUT, key as u32, down_or_up, 0 as char),
//         }
//     }
//     if down_or_up == 2 {  // up
//         match key {
//             1 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_LEFTUP, 0, 0 as char),
//             2 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_RIGHTUP, 0, 0 as char),
//             4 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_MIDDLEUP, 0, 0 as char),
//             5 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_XUP, XBUTTON1, 0 as char),
//             6 => key_event(EventType::MOUSEINPUT, MOUSEEVENTF_XUP, XBUTTON2, 0 as char),
//             _ => key_event(EventType::KEYBDINPUT, key as u32, down_or_up, 0 as char),
//         }
//     }
// }

// // enums used in key_event()
// #[derive(PartialEq)]
// enum EventType {
//     MOUSEINPUT,
//     KEYBDINPUT,
// }

// // creates a mouse/keyboard event with input data then sends it to windows.
// fn key_event(event_type: EventType, key: u32, flag_or_data: u32, wscan: char) {
//     let mut pinputs = match event_type {
//         EventType::MOUSEINPUT => {
//             INPUT {
//                 r#type: INPUT_MOUSE,
//                 Anonymous: INPUT_0 {
//                     mi: MOUSEINPUT {
//                         dx: 0,
//                         dy: 0,
//                         mouseData: flag_or_data as u32, // usualy zero | MOUSEEVENTF_WHEEL | MOUSEEVENTF_XDOWN | MOUSEEVENTF_XUP
//                         dwFlags: key as u32,            // MOUSEEVENTF_(LEFT,RIGHT,MIDDLE,X,)(DOWN OR UP)
//                         time: 0,
//                         dwExtraInfo: 0, 
//                     }
//                 }
//             }
//         },
//         EventType::KEYBDINPUT => {
//             INPUT {
//                 r#type: INPUT_KEYBOARD,
//                 Anonymous: INPUT_0 {
//                     ki: KEYBDINPUT {
//                         wVk: key as u16,         // zero for unicode | else use VK_????
//                         wScan: wscan as u16,     // chars / eg. 'a' 'b' 'c'
//                         dwFlags: flag_or_data,   // 0 = vk down / 2 = vk up / KEYEVENTF_UNICODE
//                         time: 0,
//                         dwExtraInfo: 0,
//                     }
//                 }
//             }
//         }
//     };

//     // easy reading of 'sendinput' variables
//     let cinputs = 1;
//     let cbsize = std::mem::size_of::<INPUT>() as i32;

//     // call windows api to do the magic
//     unsafe { SendInput(cinputs, &mut pinputs, cbsize); }
// }


// // keyboard hook
// // unsafe extern "system" fn kbd_hook(n_code: i32, w_param: usize, l_param: isize) -> isize {
// //     let s: *mut KBDLLHOOKSTRUCT = l_param as _;

// //     match w_param as u32 {
// //       WM_KEYDOWN => {
// //         let c: u8 = MapVirtualKeyW((*s).vkCode, MAPVK_VK_TO_CHAR) as u8;
// //         println!("{}", c as char);
// //       }
// //       _ => (),
// //     }

// //     CallNextHookEx(None, n_code, w_param, l_param)
// // }