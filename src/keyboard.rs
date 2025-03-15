use spin::Mutex;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use crate::window_manager::WINDOW_MANAGER;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
}

static mut COMMAND_BUFFER: [char; 64] = ['\0'; 64];
static mut BUFFER_POS: usize = 0;
static mut CURSOR_Y: u16 = 1;

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
}

fn process_character(c: char) {
    unsafe {
        let mut manager = WINDOW_MANAGER.lock();
        if let Some(terminal_window) = manager.get_active_window() {
            if c == '\n' {
                let mut command_str = [0u8; 64];
                for (i, &ch) in COMMAND_BUFFER[0..BUFFER_POS].iter().enumerate() {
                    command_str[i] = ch as u8;
                }
                let command = core::str::from_utf8(&command_str[..BUFFER_POS]).unwrap_or("");
                
                BUFFER_POS = 0;
                for i in 0..64 {
                    COMMAND_BUFFER[i] = '\0';
                }
                
                CURSOR_Y += 1;
                drop(manager);
                crate::command::execute_command(command, CURSOR_Y as u16);
                CURSOR_Y += 2;
            } else {
                COMMAND_BUFFER[BUFFER_POS] = c;
                BUFFER_POS += 1;
                
                let c_str = [c as u8];
                terminal_window.print_at(
                    (BUFFER_POS + 1) as usize,
                    CURSOR_Y as usize,
                    core::str::from_utf8(&c_str).unwrap_or("")
                );
            }
        }
    }
}