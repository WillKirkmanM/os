use crate::vga::{Color, ColorCode, WRITER};
use spin::Mutex;

const TOP_LEFT: u8 = b'+';
const TOP_RIGHT: u8 = b'+';
const BOTTOM_LEFT: u8 = b'+';
const BOTTOM_RIGHT: u8 = b'+';
const HORIZONTAL: u8 = b'-';
const VERTICAL: u8 = b'|';

#[derive(Debug, Clone, Copy)]
pub struct Window {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub title: &'static str,
    pub color: Color,
}

impl Window {
    pub fn new(x: usize, y: usize, width: usize, height: usize, title: &'static str, color: Color) -> Self {
        Window {
            x,
            y,
            width,
            height,
            title,
            color,
        }
    }

    pub fn draw(&self) {
        let mut writer = WRITER.lock();
        let color_code = ColorCode::new(self.color, Color::Black);
        let original_color = writer.color_code;
        
        writer.color_code = color_code;
        
        writer.write_char_at(self.x, self.y, TOP_LEFT);
        for x in (self.x + 1)..(self.x + self.width - 1) {
            writer.write_char_at(x, self.y, HORIZONTAL);
        }
        writer.write_char_at(self.x + self.width - 1, self.y, TOP_RIGHT);
        
        if self.title.len() < self.width - 2 {
            let title_start = self.x + (self.width - self.title.len()) / 2;
            for (i, &byte) in self.title.as_bytes().iter().enumerate() {
                writer.write_char_at(title_start + i, self.y, byte);
            }
        }
        
        for y in (self.y + 1)..(self.y + self.height - 1) {
            writer.write_char_at(self.x, y, VERTICAL);
            writer.write_char_at(self.x + self.width - 1, y, VERTICAL);
            
            for x in (self.x + 1)..(self.x + self.width - 1) {
                writer.write_char_at(x, y, b' ');
            }
        }
        
        writer.write_char_at(self.x, self.y + self.height - 1, BOTTOM_LEFT);
        for x in (self.x + 1)..(self.x + self.width - 1) {
            writer.write_char_at(x, self.y + self.height - 1, HORIZONTAL);
        }
        writer.write_char_at(self.x + self.width - 1, self.y + self.height - 1, BOTTOM_RIGHT);
        
        writer.color_code = original_color;
    }
    
    pub fn print_at(&self, x_offset: usize, y_offset: usize, text: &str) {
        let start_x = self.x + 1 + x_offset;
        let start_y = self.y + 1 + y_offset;
        
        if start_y >= self.y + self.height - 1 {
            return;
        }
        
        let mut writer = WRITER.lock();
        let original_color = writer.color_code;
        writer.color_code = ColorCode::new(self.color, Color::Black);
        
        for (i, &byte) in text.as_bytes().iter().enumerate() {
            let x = start_x + i;
            if x >= self.x + self.width - 1 {
                break;
            }
            if x < 80 && start_y < 25 {
                writer.write_char_at(x, start_y, byte);
            }
        }
        
        writer.color_code = original_color;
    }
}

pub struct WindowManager {
    windows: [Option<Window>; 10],
    active_window: usize,
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager {
            windows: [None; 10],
            active_window: 0,
        }
    }
    
    pub fn add_window(&mut self, window: Window) -> Option<usize> {
        for (i, slot) in self.windows.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(window);
                return Some(i);
            }
        }
        None
    }
    
    pub fn get_window(&self, id: usize) -> Option<Window> {
        self.windows.get(id).and_then(|w| *w)
    }
    
    pub fn set_active_window(&mut self, id: usize) {
        if id < self.windows.len() && self.windows[id].is_some() {
            self.active_window = id;
        }
    }
    
    pub fn draw_all(&self) {
        let active = self.active_window;
        
        for (i, window) in self.windows.iter().enumerate() {
            if i != active {
                if let Some(window) = window {
                    window.draw();
                }
            }
        }
        
        if let Some(window) = self.windows.get(active).and_then(|w| *w) {
            window.draw();
        }
    }
}

lazy_static::lazy_static! {
    pub static ref WINDOW_MANAGER: Mutex<WindowManager> = Mutex::new(WindowManager::new());
}