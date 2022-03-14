use hotkey::*;

// a few examples

fn main() {

    // block a key
    Hotkey::new(VK_1).spawn(OnDown);

    // send a different key
    Hotkey::new(VK_2).send(VK_A).spawn(OnDown);

    // send a different key on release
    Hotkey::new(VK_3).send(VK_B).spawn(OnUp);

    // switch right alt for control
    Hotkey::new(VK_RMENU).down(VK_CONTROL).spawn(OnDown);
    Hotkey::new(VK_RMENU).up(VK_CONTROL).spawn(OnUp);

    // require modifier keys
    // hold f1 + f2 + f3 + f4 before pressing '4'
    Hotkey::new(VK_4).mods(vec![VK_F1, VK_F2, VK_F3, VK_F4]).send(VK_C).spawn(OnDown);

    // send code
    Hotkey::new(VK_5).code(test).spawn(OnDown);
    fn test() {
        println!("print to console");
        VK_D.send();
    }

    // send code with variable
    Hotkey::new(VK_6).code(|| test_var(4, 5) ).spawn(OnDown);
    fn test_var(x: i32, y: i32) {
        println!("{} + {} = {}", x, y, x + y);
        VK_E.send();
    }

    // block injected events from triggering - disabled (default)
    Hotkey::new(VK_7).code(block7).spawn(OnUp);
    fn block7() {
        println!("block inject disabled");
        VK_7.send();
    }

    // block injected events from triggering - enabled
    Hotkey::new(VK_8).block_inject().code(block8).spawn(OnUp);
    fn block8() {
        println!("block inject enabled");
        VK_8.send();
    }

    // set hooks and start message loop
    set_hook();
}