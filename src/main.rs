
mod window;

fn main() {
    let bounds;
    {
        let mut window = window::Window::new().unwrap();
        window.clear();
        bounds = window.bounds();

        for i in 0..window.bounds().x {
            window.set_cursor(&window::position::Coordinates::from((i, 0)));
            std::thread::sleep(std::time::Duration::from_millis(25));
        }

        window.put_string(
            "This is a test message",
            &Some(window::position::Coordinates::from((0, 1))),
            &Some(window::color::ColorPair::new(
                1,
                window::color::TermColor::Blue,
                window::color::TermColor::White
            ).expect("Failed to initialize color"))
        );
        
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    println!("Window bounds were: {bounds:?}");
    println!("Program complete.")
}
