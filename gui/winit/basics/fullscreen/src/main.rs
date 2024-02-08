use simple_logger::SimpleLogger;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::{Fullscreen, WindowBuilder},
};

#[path = "util/fill.rs"]
mod fill;

fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new().unwrap();

    let mut decorations = true;
    let mut minimized = false;
    let mut with_min_size = false;
    let mut with_max_size = false;

    let window = WindowBuilder::new()
        .with_title("Fullscreen Mode.")
        .build(&event_loop)
        .unwrap();

    let mut monitor_index = 0;
    let mut monitor = event_loop
        .available_monitors()
        .next()
        .expect("No monitors found.")
    println!("Monitor: {:?}", monitor.name());

    let mut mode_index = 0;
    let mut mode = monitor
        .video_modes()
        .next()
        .expect("No modes found.");
    println!("Mode: {mode}");

    println!("Keys:");
    println!("- Esc\tExit");
    println!("- F\tToggle exclusive fullscreen mode");
    println!("- B\tToggle borderless mode");
    println!("- C\tToggle simple fullscreen mode");
    println!("- S\tNext screen");
    println!("- M\tNext mode for this screen");
    println!("- D\tToggle window decorations");
    println!("- X\tMaximize window");
    println!("- Z\tMinimize window");
    println!("- I\tToggle mIn size limit");
    println!("- A\tToggle mAx size limit");

    event_loop.run(move |event, elwt) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: key,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match key {
                    Key::Named(NamedKey::Escape) => elwt.exit(),
                    
                }
            }
        }
    }
}
