use sdl3::event::Event;
use sdl3::image::LoadSurface;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{Canvas, Texture, TextureCreator};
use sdl3::surface::Surface;
use sdl3::video::{Window, WindowContext};
use sdl3::{Sdl, VideoSubsystem};
use std::path::PathBuf;
use std::time::Duration;

use crate::files::ImageViewer;

mod files;
mod images;

struct AppState {
    sdl_context: Sdl,
    sdl_video: VideoSubsystem,
    window: Window,
    window_canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    viewer: ImageViewer,
    root: PathBuf,
}

impl AppState {
    fn init(path: &'static str) -> anyhow::Result<AppState> {
        let sdl_context = sdl3::init()?;
        let sdl_video = sdl_context.video()?;

        let window = sdl_video
            .window("imvi", 800, 600)
            .position_centered()
            .resizable()
            .build()?;
        let mut window_canvas = window.clone().into_canvas();
        window_canvas.set_draw_color(Color::RGB(23, 36, 42));

        let root = PathBuf::from(path);
        let viewer = ImageViewer::load(&root)?;

        Ok(AppState {
            sdl_context,
            sdl_video,
            window,
            texture_creator: window_canvas.texture_creator(),
            window_canvas,

            viewer,
            root: root.clone(),
        })
    }

    fn run(&mut self) -> anyhow::Result<()> {
        self.viewer.load_thumbnails(&self.texture_creator)?;
        self.viewer.change_image(&self.texture_creator)?;

        let mut event_pump = self.sdl_context.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::MouseWheel { y, .. } => {
                        if y < 0.0 {
                            self.viewer.next(&self.texture_creator)?;
                        }
                        if y > 0.0 {
                            self.viewer.prev(&self.texture_creator)?;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        self.viewer.next(&self.texture_creator)?;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        self.viewer.prev(&self.texture_creator)?;
                    }
                    _ => {}
                }
            }

            self.window_canvas.clear();
            self.viewer.draw(&mut self.window_canvas)?;
            self.window_canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}

fn main() {
    let mut app = AppState::init("./test_data").unwrap();
    app.run().unwrap();
}
