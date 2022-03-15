// #![windows_subsystem = "windows"]
use hotkey::*;

fn main() {
    hotkey_mode();  // set tipical hotkey layout    
    set_hook();     // loop
} 

fn hotkey_mode() {
    Hotkey::remove_all();

    keys_up();  // ensure keys are up
    capslock(); // set caps lock as a special key
    keyboard(); // keyboard hotkeys
    mouse();    // mouse hotkeys
}

fn capslock() {
    // use caps lock
    Hotkey::new(VK_CAPITAL).mods(vec![VK_CONTROL]).send(VK_CAPITAL).spawn(OnDown);
    
    // on caps lock down
    Hotkey::new(VK_CAPITAL).code(|| {
        SetKeyState::toggle_off(VK_NUMLOCK);
        SetKeyState::down(VK_F24);
    }).spawn(OnDown);
    
    // on caps lock up
    Hotkey::new(VK_CAPITAL).code(|| {
        SetKeyState::toggle_on(VK_NUMLOCK);  
        keys_up();
    }).spawn(OnUp);
}

fn keyboard() {
    // other modes
    Hotkey::new(VK_BACK).mods(vec![VK_F24, VK_F23]).code(unhook).spawn(OnDown);  // game mode


    // switch right alt for control
    Hotkey::new(VK_RMENU).down(VK_CONTROL).spawn(OnDown);
    Hotkey::new(VK_RMENU).up(VK_CONTROL).spawn(OnUp);

    // mod keys
    Hotkey::new(VK_Q).mods(vec![VK_F24]).down(VK_F23)    .spawn(OnDown);    // special f23
    Hotkey::new(VK_Q).mods(vec![VK_F24]).  up(VK_F23)    .spawn(OnUp);      // special f23
    Hotkey::new(VK_A).mods(vec![VK_F24]).down(VK_CONTROL).spawn(OnDown);    // control
    Hotkey::new(VK_A).mods(vec![VK_F24]).  up(VK_CONTROL).spawn(OnUp);      // control
    Hotkey::new(VK_S).mods(vec![VK_F24]).down(VK_MENU)   .spawn(OnDown);    // alt
    Hotkey::new(VK_S).mods(vec![VK_F24]).  up(VK_MENU)   .spawn(OnUp);      // alt
    Hotkey::new(VK_D).mods(vec![VK_F24]).down(VK_SHIFT)  .spawn(OnDown);    // shift
    Hotkey::new(VK_D).mods(vec![VK_F24]).  up(VK_SHIFT)  .spawn(OnUp);      // shift

    // large movement: special
    Hotkey::new(VK_I).mods(vec![VK_F24, VK_F23]).send(VK_PRIOR).spawn(OnDown);  // page up
    Hotkey::new(VK_K).mods(vec![VK_F24, VK_F23]).send(VK_NEXT).spawn(OnDown);   // page down
    Hotkey::new(VK_J).mods(vec![VK_F24, VK_F23]).send(VK_HOME).spawn(OnDown);   // home
    Hotkey::new(VK_L).mods(vec![VK_F24, VK_F23]).send(VK_END).spawn(OnDown);    // end

    // small movement
    Hotkey::new(VK_I).mods(vec![VK_F24]).send(VK_UP).spawn(OnDown);     // up
    Hotkey::new(VK_K).mods(vec![VK_F24]).send(VK_DOWN).spawn(OnDown);   // down
    Hotkey::new(VK_J).mods(vec![VK_F24]).send(VK_LEFT).spawn(OnDown);   // left
    Hotkey::new(VK_L).mods(vec![VK_F24]).send(VK_RIGHT).spawn(OnDown);  // right


    Hotkey::new(VK_U).mods(vec![VK_F24]).send(VK_BACK).spawn(OnDown);    // backspace
    Hotkey::new(VK_O).mods(vec![VK_F24]).send(VK_DELETE).spawn(OnDown);  // delete
}

fn mouse() {
    // copy
    Hotkey::new(VK_LBUTTON).mods(vec![VK_F24]).code(|| {
        [VK_CONTROL.down(), VK_C.send(), VK_CONTROL.up()];
    }).spawn(OnDown);
    Hotkey::new(VK_LBUTTON).mods(vec![VK_F24]).spawn(OnUp);

    // paste
    Hotkey::new(VK_RBUTTON).mods(vec![VK_F24]).code(|| {
        [VK_CONTROL.down(), VK_V.send(), VK_CONTROL.up()];
    }).spawn(OnDown);
    Hotkey::new(VK_RBUTTON).mods(vec![VK_F24]).spawn(OnUp);

    // undo
    Hotkey::new(VK_XBUTTON1).mods(vec![VK_F24]).code(|| {
        [VK_CONTROL.down(), VK_Z.send(), VK_CONTROL.up()];
    }).spawn(OnDown);
    Hotkey::new(VK_XBUTTON1).mods(vec![VK_F24]).spawn(OnUp);

    // redo
    Hotkey::new(VK_XBUTTON2).mods(vec![VK_F24]).code(|| {
        [VK_CONTROL.down(), VK_Y.send(), VK_CONTROL.up()];
    }).spawn(OnDown);
    Hotkey::new(VK_XBUTTON2).mods(vec![VK_F24]).spawn(OnUp);
}

fn keys_up() {
    SetKeyState::up(VK_CONTROL);
    SetKeyState::up(VK_MENU);
    SetKeyState::up(VK_SHIFT);
    SetKeyState::up(VK_F23);
    SetKeyState::up(VK_F24);
}

// close the program
fn unhook() {
    std::process::exit(0);
}

struct SetKeyState;

impl SetKeyState {
    fn up(key: VIRTUAL_KEY) { if key.is_down() { key.up() } }
    fn down(key: VIRTUAL_KEY) { if !key.is_down() { key.down() } }
    //fn switch(key: VIRTUAL_KEY) { if key.is_down() { key.up() } if !key.is_down() { key.down() } }
    fn toggle_on(key: VIRTUAL_KEY) { if !key.is_toggle_on() { key.send() } }
    fn toggle_off(key: VIRTUAL_KEY) { if key.is_toggle_on() { key.send() } }
}


