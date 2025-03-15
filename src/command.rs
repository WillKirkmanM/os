use crate::window_manager::WINDOW_MANAGER;

pub fn execute_command(command: &str, cursor_y: u16) {
    let mut manager = WINDOW_MANAGER.lock();
    
    if let Some(terminal_window) = manager.get_active_window() {
        let response = match command.trim() {
            "help" => "Available commands: help, clear, info",
            "clear" => {
                terminal_window.clear();
                ""
            },
            "info" => "OS Version 0.1.0",
            "" => "",
            _ => "Unknown command. Type 'help' for available commands.",
        };
        
        if !response.is_empty() {
            terminal_window.print_at(1, (cursor_y + 1).into(), response);
        }
    }
}