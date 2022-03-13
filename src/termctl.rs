
pub fn clear_screen() {
    print!("\x1b[2J");
    print!("\x1b[H");
}

