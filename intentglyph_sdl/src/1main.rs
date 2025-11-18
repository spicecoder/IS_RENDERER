use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("IntentGlyph Viewer", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        // Draw a yellow rectangle as a "cell"
        canvas.set_draw_color(Color::RGB(255, 255, 0));
        canvas.fill_rect(Rect::new(100, 100, 200, 150))?;

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
