use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        
        idt[InterruptIndex::Timer as usize]
            .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as usize]
            .set_handler_fn(keyboard_interrupt_handler);
        
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    use x86_64::instructions::port::Port;
    
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    
    crate::keyboard::handle_keyboard_interrupt(scancode);
    
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}