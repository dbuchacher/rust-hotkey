use hotkey::*;
use std::{thread::sleep, time::Duration};


// all VK codes are here
    // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes


fn main() {

    // will open the windows menu by pressing the windows key
    VK_LWIN.down();
    VK_LWIN.up();

    // after waiting 3 seconds
    sleep(Duration::from_secs(3));

    // the windows menu will be closed from this pressing the windows key
    VK_LWIN.send();


    // the a key should be stuck down showing "a key is down"
    // at least until it gets phsicaly pushed on the keyboard
    VK_A.down();

    // the console will keep printing a message about the 'A' key state
    loop {
        if VK_A.is_down() {
            println!("'a' key is down");
        } else if VK_A.is_up() {
            println!("'a' key is up");
        }
    }
}