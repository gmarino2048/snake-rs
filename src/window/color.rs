
use super::error::{ WindowError, WindowErrorType };

#[repr(i16)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TermColor {
    Black   = ncurses::COLOR_BLACK,
    Red     = ncurses::COLOR_RED,
    Green   = ncurses::COLOR_GREEN,
    Yellow  = ncurses::COLOR_YELLOW,
    Blue    = ncurses::COLOR_BLUE,
    Magenta = ncurses::COLOR_MAGENTA,
    Cyan    = ncurses::COLOR_CYAN,
    White   = ncurses::COLOR_WHITE
}

#[derive(Debug, Clone)]
pub struct ColorPair {
    id: i16,

    foreground: TermColor,
    background: TermColor
}

#[allow(dead_code)]
impl ColorPair {
    pub fn new(id: i16, foreground: TermColor, background: TermColor) -> Result<Self, WindowError> {
        if ncurses::init_pair(id, foreground as i16, background as i16) != ncurses::OK {
            return Err(WindowError::new(
                WindowErrorType::Color,
                format!("Failed to initialize color pair with id {id:?}")
            ));
        }

        Ok(ColorPair { 
            id: id,
            foreground: foreground,
            background: background
        })
    }

    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn foreground(&self) -> TermColor {
        self.foreground
    }

    pub fn background(&self) -> TermColor {
        self.background
    }
}
