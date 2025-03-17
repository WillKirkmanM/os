#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use core::fmt::{self, Write};
mod vga;
mod window_manager;

struct ByteWriter<'a> {
    buf: &'a mut [u8],
    cursor: usize,
}

impl<'a> ByteWriter<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        ByteWriter { buf, cursor: 0 }
    }
}

impl<'a> fmt::Write for ByteWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len().min(self.buf.len() - self.cursor);
        self.buf[self.cursor..self.cursor + len].copy_from_slice(&bytes[..len]);
        self.cursor += len;
        Ok(())
    }
}
mod interrupts;
mod keyboard;
mod command;
mod terminal;
mod config;
mod graphics;

use vga::Color;
use window_manager::{Window, WINDOW_MANAGER};
use config::UiMode;
use graphics::{Renderer3D, create_cube};

pub fn launch_3d_demo() -> Option<usize> {
    let window = Window::new(15, 4, 50, 20, "3D Graphics Demo", Color::LightBlue);
    
    let mut manager = WINDOW_MANAGER.lock();
    let window_id = manager.add_window(window)?;
    manager.set_active_window(window_id);
    manager.draw_all();
    
    let window = manager.get_window(window_id)?;
    drop(manager);
    
    let mut renderer = Renderer3D::new();
    let cube = create_cube();
    
    let mut frame_counter = 0;
    let rotation_speed = 0.05;
    
    window.print_at(1, 18, "Press ESC to close the demo...");

    loop {
        if let Some(true) = keyboard::is_esc_pressed() {
            break;
        }
        
        for y in 2..18 {
            for x in 2..48 {
                window.print_at(x, y, " ");
            }
        }
        
        renderer.rotate(rotation_speed, rotation_speed * 1.5, rotation_speed * 0.7);
        
        renderer.render_object(&window, &cube);
        
        for _ in 0..300000 {
            core::hint::spin_loop();
        }
        
        frame_counter += 1;
        let mut frame_str = [0u8; 16];
        write!(ByteWriter::new(&mut frame_str), "Frame: {}", frame_counter).unwrap();
        window.print_at(32, 1, core::str::from_utf8(&frame_str).unwrap());
    }
    
    Some(window_id)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    
    match config::get_current_ui_mode() {
        UiMode::Windowed => {
            vga::WRITER.lock().clear_screen();
            
            let window1 = Window::new(5, 3, 30, 10, "Main Window", Color::Cyan);
            let window2 = Window::new(40, 5, 35, 8, "System Info", Color::Green);
            let window3 = Window::new(20, 15, 40, 6, "Terminal", Color::White);
            
            let mut manager = WINDOW_MANAGER.lock();
            let id1 = manager.add_window(window1).unwrap();
            let id2 = manager.add_window(window2).unwrap();
            let id3 = manager.add_window(window3).unwrap();
            
            manager.set_active_window(id3);
            manager.draw_all();
            
            let window1 = manager.get_window(id1).unwrap();
            window1.print_at(1, 3, "Window Manager Demo");
            
            let window2 = manager.get_window(id2).unwrap();
            window2.print_at(1, 1, "CPU: x86_64");
            window2.print_at(1, 2, "Memory: 64MB");
            window2.print_at(1, 3, "Status: Running");
            
            let terminal_window = manager.get_window(id3).unwrap();
            drop(manager);
            
            terminal::init_terminal_windowed(terminal_window);
        },
        UiMode::FullscreenTerminal => {
            terminal::init_terminal_fullscreen();
        }
    }
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}