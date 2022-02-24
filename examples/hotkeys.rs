use hotkey::*;

// a few examples

fn main() {

    // block
    // pressing '1' results in 'nothing'
    // blocks the initial key
    Hotkey::new(VK_1)
        .block()
        .spawn();

    // send
    // pressing '2' results in '2a'
    // send the initial key and
    // send the second key
    Hotkey::new(VK_2)
        .send(VK_A)
        .spawn();

    // send + block
    // pressing '3' results in 'b'
    // block the initial key
    // send the second key
    Hotkey::new(VK_3)
        .send(VK_B)
        .block()
        .spawn();

    // on release + send
    // pressing '4' results '4c'
    // holding '4' and releasing after a second will result in something like '44444444444c'
    Hotkey::new(VK_4)
        .on_release()
        .send(VK_C)
        .spawn();      

    // code
    // pressing '5' results in something like '5555555555555'
    // also hello world gets printed to console
    fn five() {
        println!("hello world");
        VK_5.send();
    }
    Hotkey::new(VK_5)
        .code(five)
        .spawn();

    // code + block_inject
    // pressing '6' results in '66'  (add block() if you want '6')
    fn six() {
        println!("hello world");
        VK_6.send();
    }
    Hotkey::new(VK_6)
        .code(six)
        .block_inject()
        .spawn();

    // add_mods + send + block
    // pressing '7' results in '7'
    // holding 'A'+'S'+'D'+'F' then pressing '7' will result in 'x'
    Hotkey::new(VK_7)
        .add_mods(vec![VK_A, VK_S, VK_D, VK_F])
        .send(VK_X)
        .block()
        .spawn();


    // set hooks and start message loop
    set_hook();
}
