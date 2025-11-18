use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas,  TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use sdl2::{image::LoadTexture};
use std::fs;
use std::path::Path;
use sdl2::render::TextureQuery;

use serde::Deserialize;

#[derive(Deserialize)]
struct Scene {
    width: u32,
    height: u32,
    layers: Vec<Layer>,
}

#[derive(Deserialize)]
struct Layer {
    grid_rows: u32,
    grid_cols: u32,
    cells: Vec<Cell>,
}

#[derive(Deserialize)]
struct Cell {
    row: u32,
    col: u32,
    span_rows: u32,
    span_cols: u32,
    content: Option<String>,
    image: Option<String>,
}

fn render_cell<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &Font<'a, 'static>,
    cell: &Cell,
    layer: &Layer,
    window_width: u32,
    window_height: u32,
) {
    let cell_width = window_width / layer.grid_cols;
    let cell_height = window_height / layer.grid_rows;

    //let x = (cell.col - 1) * cell_width;
    let x = cell.col.saturating_sub(1) * cell_width;

    //let y = (cell.row - 1) * cell_height;
    let y = cell.row.saturating_sub(1) * cell_height;

    let w = cell.span_cols * cell_width;
    let h = cell.span_rows * cell_height;

    let rect = Rect::new(x as i32, y as i32, w, h);
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.fill_rect(rect).ok();

    if let Some(ref img_path) = cell.image {
        println!("Loading image: {}", img_path);
        if let Ok(texture) = texture_creator.load_texture(Path::new(img_path)) {
            canvas.copy(&texture, None, rect).ok();
        }
    }

    if let Some(ref content) = cell.content {
        let surface = font
            .render(content)
            .blended(Color::WHITE)
            .expect("Text render failed");
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .expect("Texture creation failed");
        let TextureQuery { width, height, .. } = texture.query();
        let dst = Rect::new(x as i32 + 10, y as i32 + 10, width, height);
        canvas.copy(&texture, None, Some(dst)).ok();
    }
}

fn main() -> Result<(), String> {
    let scene_json = fs::read_to_string("scene.json").expect("Failed to read scene.json");
    let scene: Scene = serde_json::from_str(&scene_json).expect("Invalid JSON");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("IntentGlyph Viewer", scene.width, scene.height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let font_path = "/Library/Fonts/Arial Unicode.ttf"; // Or update to a known existing path
    let font = ttf_context.load_font(font_path, 24).unwrap();

    canvas.set_draw_color(Color::RGB(20, 20, 20));
    canvas.clear();

    for layer in &scene.layers {
        for cell in &layer.cells {
            render_cell(
                &mut canvas,
                &texture_creator,
                &font,
                cell,
                layer,
                scene.width,
                scene.height,
            );
        }
    }

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'running;
            }
        }
    }

    Ok(())
}
