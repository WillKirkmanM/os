use crate::window_manager::Window;
use crate::vga::{Color, WRITER, ColorCode};
use spin::Mutex;
use lazy_static::lazy_static;

const MAX_HISTORY: usize = 10;
const MAX_HISTORY_LINES: usize = 100;

pub enum TerminalOutput {
    Windowed(Window),
    Fullscreen,
}

pub struct Terminal {
    pub output: TerminalOutput,
    history: [Option<[char; 64]>; MAX_HISTORY],
    history_lines: [[char; 64]; MAX_HISTORY_LINES],
    history_index: usize,
    current_line: usize,
    prompt: &'static str,
}

impl Terminal {
    pub fn new_windowed(window: Window) -> Self {
        let prompt = "$ ";
        let mut terminal = Terminal {
            output: TerminalOutput::Windowed(window),
            history: [None; MAX_HISTORY],
            history_lines: [['\0'; 64]; MAX_HISTORY_LINES],
            history_index: 0,
            current_line: 0,
            prompt,
        };
        
        terminal.print_at(1, 1, prompt);
        terminal
    }
    
    pub fn new_fullscreen() -> Self {
        WRITER.lock().clear_screen();
        
        let prompt = "$ ";
        let mut terminal = Terminal {
            output: TerminalOutput::Fullscreen,
            history: [None; MAX_HISTORY],
            history_lines: [['\0'; 64]; MAX_HISTORY_LINES],
            history_index: 0,
            current_line: 0,
            prompt,
        };
        
        terminal.print_at(1, 0, prompt);
        terminal
    }
    
    pub fn handle_input(&mut self, c: char, buffer: &mut [char; 64], buffer_pos: &mut usize) {
        match c {
            '\n' => self.process_command(buffer, buffer_pos),
            '\u{0008}' => self.handle_backspace(buffer, buffer_pos),
            _ => self.handle_character(c, buffer, buffer_pos),
        }
    }
    
    fn handle_character(&mut self, c: char, buffer: &mut [char; 64], buffer_pos: &mut usize) {
        if *buffer_pos < buffer.len() - 1 {
            buffer[*buffer_pos] = c;
            *buffer_pos += 1;
            
            let c_str = [c as u8];
            self.print_at(
                1 + self.prompt.len() + *buffer_pos - 1,
                self.current_line,
                core::str::from_utf8(&c_str).unwrap_or("")
            );
        }
    }
    
    fn handle_backspace(&mut self, buffer: &mut [char; 64], buffer_pos: &mut usize) {
        if *buffer_pos > 0 {
            *buffer_pos -= 1;
            buffer[*buffer_pos] = '\0';
            
            self.print_at(
                1 + self.prompt.len() + *buffer_pos,
                self.current_line,
                " "
            );
        }
    }
    
    fn process_command(&mut self, buffer: &mut [char; 64], buffer_pos: &mut usize) {
        let mut command_str = [0u8; 64];
        for (i, &ch) in buffer[0..*buffer_pos].iter().enumerate() {
            command_str[i] = ch as u8;
        }
        let command = core::str::from_utf8(&command_str[..*buffer_pos]).unwrap_or("");
        
        if *buffer_pos > 0 {
            let mut history_entry = ['\0'; 64];
            for (i, &ch) in buffer[0..*buffer_pos].iter().enumerate() {
                history_entry[i] = ch;
            }
            self.add_to_history(history_entry);
        }
        
        for i in 0..buffer.len() {
            buffer[i] = '\0';
        }
        *buffer_pos = 0;
        
        self.current_line += 1;
        let response = self.execute_command(command);
        
        if !response.is_empty() {
            self.print_at(1, self.current_line, response);
            self.current_line += 1;
        }
        
        self.scroll_if_needed();
        
        self.print_at(1, self.current_line, self.prompt);
    }
    
    fn add_to_history(&mut self, entry: [char; 64]) {
        for i in (1..MAX_HISTORY).rev() {
            self.history[i] = self.history[i-1];
        }
        self.history[0] = Some(entry);
    }
    
    
    
    pub fn clear(&mut self) {
        match &self.output {
            TerminalOutput::Windowed(window) => window.clear(),
            TerminalOutput::Fullscreen => WRITER.lock().clear_screen(),
        }
        self.current_line = 0;
    }
    
    fn print_at(&mut self, x_offset: usize, y_offset: usize, text: &str) {
        match &self.output {
            TerminalOutput::Windowed(window) => {
                window.print_at(x_offset, y_offset, text);
            },
            TerminalOutput::Fullscreen => {
                let mut writer = WRITER.lock();
                let original_color = writer.color_code;
                writer.color_code = ColorCode::new(Color::White, Color::Black);
                
                for (i, &byte) in text.as_bytes().iter().enumerate() {
                    let x = x_offset + i;
                    writer.write_char_at(x, y_offset, byte);
                }
                
                writer.color_code = original_color;
            }
        }
    }
    
    fn scroll_if_needed(&mut self) {
        let max_height = match &self.output {
            TerminalOutput::Windowed(window) => window.height - 2,
            TerminalOutput::Fullscreen => 25,
        };
        
        if self.current_line >= max_height {
            self.clear();
            self.current_line = 0;
        }
    }
}

lazy_static! {
    pub static ref TERMINAL: Mutex<Option<Terminal>> = Mutex::new(None);
}

pub fn init_terminal_windowed(window: Window) {
    let terminal = Terminal::new_windowed(window);
    *TERMINAL.lock() = Some(terminal);
}

pub fn init_terminal_fullscreen() {
    let terminal = Terminal::new_fullscreen();
    *TERMINAL.lock() = Some(terminal);
}
