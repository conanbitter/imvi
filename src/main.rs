use std::path::PathBuf;

use crate::files::ImageContainer;
use crate::image_viewer::ImageViewer;
use crate::window::EventHandler;
use crate::window::Keycode;
use crate::window::MouseButton;

mod files;
mod grid_viewer;
mod image_viewer;
mod images;
mod window;

struct AppState {
    images: ImageContainer,
    image_viewer: ImageViewer,
}

impl AppState {
    fn next(&mut self) -> anyhow::Result<()> {
        if self.images.next() {
            self.update_state()?;
        }
        Ok(())
    }

    fn prev(&mut self) -> anyhow::Result<()> {
        if self.images.prev() {
            self.update_state()?;
        }
        Ok(())
    }

    fn update_state(&mut self) -> anyhow::Result<()> {
        /*self.window.set_title(
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
        self.update_view();*/
        Ok(())
    }

    fn init(path: &'static str) -> anyhow::Result<AppState> {
        let root = PathBuf::from(path);
        let images = ImageContainer::load(&root)?;

        Ok(AppState {
            images,
            image_viewer: ImageViewer::new(),
        })
    }
}

impl EventHandler for AppState {
    fn load(&mut self, window: &mut window::Window) {}

    fn update(&mut self, window: &mut window::Window) {}

    fn draw(&mut self, window: &mut window::Window) {}

    fn scroll(&mut self, window: &mut window::Window, down: bool) {}

    fn key_down(&mut self, window: &mut window::Window, key: Keycode) {
        match key {
            Keycode::Escape => window.request_exit(),
            Keycode::Left => {}
            Keycode::Right => {}
            Keycode::Up => {}
            Keycode::Down => {}
            _ => {}
        }
    }

    fn mouse_down(&mut self, window: &mut window::Window, button: MouseButton, x: f32, y: f32) {}

    fn mouse_move(&mut self, window: &mut window::Window, x: f32, y: f32) {}
}

fn run() -> anyhow::Result<()> {
    let mut window = window::Window::init(800, 600)?;
    let mut app = AppState::init("./test_data")?;
    window.run(&mut app)?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(error) => {
            window::show_error_message(&error);
        }
    }
}
