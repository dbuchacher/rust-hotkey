use ahk::*;

fn main() {

    Swap::on_up(VK_R, VK_A);
    Block::on_down(VK_WHEELDOWN);

    unsafe { set_hook(); }
}