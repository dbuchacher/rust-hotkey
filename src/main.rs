use ahk::*;


fn mine() {
    
    println!("Rbutton is down = {}", VK_RBUTTON.is_down());
    VK_A.send();
    
}

fn main() {

    Hotkey::new(VK_A)
        .swap(VK_1)
        .block()
        .spawn();

    Hotkey::new(VK_1)
        .swap(VK_2)
        .block_inject()
        .spawn();

    Hotkey::new(VK_2)
        .swap(VK_3)
        .spawn();

    Hotkey::new(VK_3)
        .swap(VK_4)
        .spawn();

    set_hook();


}