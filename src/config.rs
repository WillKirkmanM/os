use spin::Mutex;
use lazy_static::lazy_static;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UiMode {
    Windowed,
    FullscreenTerminal,
}

lazy_static! {
    static ref CURRENT_UI_MODE: Mutex<UiMode> = Mutex::new(UiMode::Windowed);
}

pub fn set_ui_mode(mode: UiMode) {
    *CURRENT_UI_MODE.lock() = mode;
}

pub fn get_current_ui_mode() -> UiMode {
    *CURRENT_UI_MODE.lock()
}