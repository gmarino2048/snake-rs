
mod window;

use window::color::{ ColorPair, TermColor };
use window::position::Coordinates;

fn main() {
    let mut window = window::Window::new().unwrap();
    window.clear();

    for i in 0..window.bounds().x {
        window.set_cursor(&Coordinates::from((i, 0)));
        std::thread::sleep(std::time::Duration::from_millis(25));
    }

    let area = window.bounds().x * window.bounds().y;
    let color = ColorPair::new(1, TermColor::Black, TermColor::White)
        .unwrap();
    window.set_cursor(&Coordinates::from((0,0)));
    for _ in 0..area {
        window.put_char('X', &None, &Some(color.clone()))
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    let fg_colors = [TermColor::Black, TermColor::Magenta, TermColor::Red,
        TermColor::Yellow, TermColor::Green, TermColor::Cyan, TermColor::Blue, TermColor::White];

    for fg in fg_colors {
        let _ = ColorPair::new(1, fg, TermColor::Black);
        window.refresh();
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    std::thread::sleep(std::time::Duration::from_secs(10));
}
