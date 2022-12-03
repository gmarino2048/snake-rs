
pub mod color;
pub mod error;
pub mod position;

use color::{ ColorPair };
use error::{ WindowError, WindowErrorType };
use position::{ Coordinates };


pub struct Window {
    color: bool,
    handle: *mut i8
}

#[allow(dead_code)]
impl Window {
    pub fn new() -> Result<Self, WindowError> {
        let window_handle = ncurses::initscr();

        if ncurses::cbreak() != ncurses::OK {
            return Err(WindowError::new(
                WindowErrorType::Initialization,
                String::from("Failed to disable line buffering")
            ));
        }

        if ncurses::noecho() != ncurses::OK {
            return Err(WindowError::new(
                WindowErrorType::Initialization,
                String::from("Failed to disable window echoing")
            ));
        }

        if ncurses::intrflush(window_handle, false) != ncurses::OK {
            return Err(WindowError::new(
                WindowErrorType::Initialization,
                String::from("Failed to disable flushing on interrupt")
            ));
        }

        if ncurses::keypad(window_handle, true) != ncurses::OK {
            return Err(WindowError::new(
                WindowErrorType::Initialization,
                String::from("Failed to enable keypad input")
            ))
        }

        let mut color = ncurses::has_colors();
        if color {
            let colors_ok =
                ncurses::can_change_color() &&
                (ncurses::start_color() == ncurses::OK);
            
            if !colors_ok {
                color = false;
            }
        }

        Ok(Window {
            color: color,
            handle: window_handle
        })
    }

    pub fn refresh(&mut self) {
        ncurses::wrefresh(self.handle);
    }

    pub fn clear(&mut self) {
        ncurses::wclear(self.handle);
        self.refresh();
    }

    pub fn bounds(&mut self) -> Coordinates {
        let mut dims = Coordinates::default();
        ncurses::getmaxyx(self.handle, &mut dims.y, &mut dims.x);

        return dims;
    }

    pub fn set_cursor(&mut self, position: &Coordinates) {
        ncurses::wmove(self.handle, position.y, position.x);
        self.refresh();
    }

    pub fn set_color(&mut self, color: &ColorPair) {
        if self.color {
            ncurses::wcolor_set(self.handle, color.id());
        }
    }

    pub fn put_char(&mut self, char: char, pos: &Option<Coordinates>, color: &Option<ColorPair>) {
        if let Some(position) = pos {
            self.set_cursor(position);
        }

        if let Some(color) = color {
            self.set_color(color)
        }

        ncurses::waddch(self.handle, char as u32);
        self.refresh();
    }

    pub fn put_string(&mut self, string: &str, pos: &Option<Coordinates>, color: &Option<ColorPair>) {
        if let Some(position) = pos {
            self.set_cursor(position);
        }

        if let Some(color) = color {
            self.set_color(color)
        }

        ncurses::waddstr(self.handle, string);
        self.refresh();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}
