use ahk::keyboard::*;


fn mine() {
    println!("hello");
    VK_0.send();
}

fn main() {



    Hotkey::new(VK_A)
        .add_mods(vec![VK_S, VK_D, VK_F])
        .block()
        .run(mine)
        .spawn();

    set_keyboard_hook();

    


}