#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
mod vga;
mod window_manager;
mod interrupts;
mod keyboard;
mod command;

use vga::Color;
use window_manager::{Window, WINDOW_MANAGER};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::WRITER.lock().clear_screen();
    
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    
    let window1 = Window::new(5, 3, 30, 10, "Main Window", Color::Cyan);
    let window2 = Window::new(40, 5, 35, 8, "System Info", Color::Green);
    let window3 = Window::new(20, 15, 40, 6, "Terminal", Color::White);
    
    let mut manager = WINDOW_MANAGER.lock();
    let id1 = manager.add_window(window1).unwrap();
    let id2 = manager.add_window(window2).unwrap();
    let id3 = manager.add_window(window3).unwrap();
    
    manager.set_active_window(id3);
    
    manager.draw_all();
    drop(manager);
    
    let window1 = WINDOW_MANAGER.lock().get_window(id1).unwrap();
    window1.print_at(1, 3, "Window Manager Demo");
    
    let window2 = WINDOW_MANAGER.lock().get_window(id2).unwrap();
    window2.print_at(1, 1, "CPU: x86_64");
    window2.print_at(1, 2, "Memory: 64MB");
    window2.print_at(1, 3, "Status: Running");
    
    let window3 = WINDOW_MANAGER.lock().get_window(id3).unwrap();
    window3.print_at(1, 1, "$ ");
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}