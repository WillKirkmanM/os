use crate::terminal::{Terminal, TerminalOutput};

impl Terminal {
    pub fn execute_command(&mut self, command: &str) -> &'static str {
        match command.trim() {
            "help" => "Available commands: help, clear, info, mode, 3d, windowed, fullscreen",
            "clear" => {
                self.clear();
                ""
            },
            "info" => "OS Version 0.1.0",
            "mode" => match self.output {
                TerminalOutput::Windowed(_) => "UI Mode: Windowed",
                TerminalOutput::Fullscreen => "UI Mode: Fullscreen Terminal",
            },
            "3d" => {
                if let TerminalOutput::Windowed(_) = self.output {
                    let _ = crate::launch_3d_demo();
                    "Launched 3D demo window"
                } else {
                    "3D demo only available in windowed mode"
                }
            },
            "windowed" => {
                if let TerminalOutput::Fullscreen = self.output {
                    crate::config::set_ui_mode(crate::config::UiMode::Windowed);
                    crate::vga::WRITER.lock().clear_screen();
                    crate::_start();
                } else {
                    "Already in windowed mode"
                }
            },
            "fullscreen" => {
                if let TerminalOutput::Windowed(_) = self.output {
                    crate::config::set_ui_mode(crate::config::UiMode::FullscreenTerminal);
                    crate::vga::WRITER.lock().clear_screen();
                    crate::_start();
                } else {
                    "Already in fullscreen mode"
                }
            },
            "" => "",
            _ => "Unknown command. Type 'help' for available commands.",
        }
        }
}
