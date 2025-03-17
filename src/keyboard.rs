use spin::Mutex;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use crate::terminal::TERMINAL;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
    static ref ESC_PRESSED: Mutex<bool> = Mutex::new(false);
}

static mut COMMAND_BUFFER: [char; 64] = ['\0'; 64];
pub static mut BUFFER_POS: usize = 0;

pub fn handle_keyboard_interrupt(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    process_character(character);
                }
                DecodedKey::RawKey(_key) => {
                }
            }
        }
    }

    if scancode == 0x01 {
        set_esc_pressed(true);
    }
}

fn process_character(c: char) {
    unsafe {
        let mut terminal_lock = TERMINAL.lock();
        if let Some(terminal) = terminal_lock.as_mut() {
            terminal.handle_input(c, &mut COMMAND_BUFFER, &mut BUFFER_POS);
        }
    }
}

pub fn set_esc_pressed(pressed: bool) {
    *ESC_PRESSED.lock() = pressed;
}

pub fn is_esc_pressed() -> Option<bool> {
    Some(*ESC_PRESSED.lock())
}