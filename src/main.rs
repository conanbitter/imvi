use sdl3::event::Event;
use sdl3::image::LoadSurface;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{Canvas, FRect, TextureCreator};
use sdl3::surface::Surface;
use sdl3::video::{Window, WindowContext};
use sdl3::{Sdl, VideoSubsystem};
use std::path::PathBuf;
use std::time::Duration;

use crate::files::ImageViewer;

mod files;
mod images;
mod window;

struct AppState {
    sdl_context: Sdl,
    sdl_video: VideoSubsystem,
    window: Window,
    window_canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    viewer: ImageViewer,
    view_rect: FRect,
    root: PathBuf,

    window_ar: f32,
    window_width: i32,
    window_height: i32,
}

impl AppState {
    fn next(&mut self) -> anyhow::Result<()> {
        if self.viewer.next() {
            self.update_state()?;
        }
        Ok(())
    }

    fn prev(&mut self) -> anyhow::Result<()> {
        if self.viewer.prev() {
            self.update_state()?;
        }
        Ok(())
    }

    fn update_view(&mut self) {
        let image_ar = self.viewer.get_image().aspect_ratio;
        if image_ar < self.window_ar {
            self.view_rect.w = self.window_height as f32 * image_ar;
            self.view_rect.h = self.window_height as f32;
            self.view_rect.x = (self.window_width as f32 - self.view_rect.w) / 2.0;
            self.view_rect.y = 0.0;
        } else {
            self.view_rect.w = self.window_width as f32;
            self.view_rect.h = self.window_width as f32 / image_ar;
            self.view_rect.x = 0.0;
            self.view_rect.y = (self.window_height as f32 - self.view_rect.h) / 2.0;
        }
    }

    fn update_state(&mut self) -> anyhow::Result<()> {
        self.window.set_title(
            format!(
                "[{}/{}] {} - imvi",
                self.viewer.index,
                self.viewer.files.len(),
                self.viewer.files[self.viewer.index].name
            )
            .as_str(),
        )?;
        let image_surface = Surface::from_file(&self.viewer.files[self.viewer.index].filename)?;
        self.viewer
            .update_image(self.viewer.index, image_surface, &self.texture_creator)?;
        self.update_view();
        Ok(())
    }

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
            view_rect: FRect::new(0.0, 0.0, 1.0, 1.0),
            root: root.clone(),

            window_ar: 800.0 / 600.0,
            window_width: 800,
            window_height: 600,
        })
    }

    fn run(&mut self) -> anyhow::Result<()> {
        self.viewer.load_thumbnails(&self.texture_creator)?;
        self.update_state()?;

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
                            self.next()?;
                        }
                        if y > 0.0 {
                            self.prev()?;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        self.next()?;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        self.prev()?;
                    }
                    _ => {}
                }
            }

            self.window_canvas.clear();
            if let Some(texture) = self.viewer.get_texture() {
                self.window_canvas.copy(texture, None, Some(self.view_rect))?;
            }
            self.window_canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}

fn main() {
    //let mut app = AppState::init("./test_data").unwrap();
    //app.run().unwrap();
    let mut window = window::Window::init(800, 600).unwrap();
    window.run().unwrap();
}
