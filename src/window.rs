use std::time::Duration;

use anyhow::Error;
use sdl3::{
    Sdl, VideoSubsystem,
    event::Event,
    messagebox::{MessageBoxFlag, show_simple_message_box},
    mouse::MouseWheelDirection,
    pixels::Color,
    render::{Canvas, Texture, TextureCreator},
    video::WindowContext,
};

pub use sdl3::keyboard::Keycode;
pub use sdl3::mouse::MouseButton;
pub use sdl3::render::FRect as Rect;

pub struct Window {
    sdl_context: Sdl,
    sdl_video: VideoSubsystem,
    window: sdl3::video::Window,
    window_canvas: Canvas<sdl3::video::Window>,
    pub texture_creator: TextureCreator<WindowContext>,

    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,

    working: bool,
}

pub trait EventHandler {
    fn load(&mut self, window: &mut Window);
    fn update(&mut self, window: &mut Window);
    fn draw(&mut self, window: &mut Window);

    fn scroll(&mut self, window: &mut Window, down: bool);
    fn key_down(&mut self, window: &mut Window, key: Keycode);
    fn mouse_down(&mut self, window: &mut Window, button: MouseButton, x: f32, y: f32);
    fn mouse_move(&mut self, window: &mut Window, x: f32, y: f32);
}

impl Window {
    pub fn init(width: u32, height: u32) -> anyhow::Result<Window> {
        let sdl_context = sdl3::init()?;
        let sdl_video = sdl_context.video()?;

        let window = sdl_video
            .window("imvi", width, height)
            .position_centered()
            .resizable()
            .build()?;
        let mut window_canvas = window.clone().into_canvas();
        window_canvas.set_draw_color(Color::RGB(23, 36, 42));

        Ok(Window {
            sdl_context,
            sdl_video,
            window,
            texture_creator: window_canvas.texture_creator(),
            window_canvas,

            width,
            height,
            aspect_ratio: width as f32 / height as f32,
            working: false,
        })
    }

    pub fn request_exit(&mut self) {
        self.working = false;
    }

    pub fn run(&mut self, handler: &mut impl EventHandler) -> anyhow::Result<()> {
        handler.load(self);

        let mut event_pump = self.sdl_context.event_pump()?;
        self.working = true;

        'running: loop {
            if !self.working {
                break;
            }
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::MouseWheel { y, direction, .. } => {
                        let down = y < 0.0;
                        let down = if direction == MouseWheelDirection::Flipped {
                            !down
                        } else {
                            down
                        };
                        handler.scroll(self, down);
                    }
                    Event::KeyDown { keycode: Some(key), .. } => {
                        handler.key_down(self, key);
                    }
                    Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                        handler.mouse_down(self, mouse_btn, x, y);
                    }
                    Event::MouseMotion { x, y, .. } => {
                        handler.mouse_move(self, x, y);
                    }
                    _ => {}
                }
            }

            handler.update(self);

            self.window_canvas.clear();
            handler.draw(self);
            self.window_canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    pub fn draw(&mut self, image: &Texture, dst: Rect) -> anyhow::Result<()> {
        self.window_canvas.copy(image, None, Some(dst))?;
        Ok(())
    }

    pub fn set_title(&mut self, title: &String) -> anyhow::Result<()> {
        self.window.set_title(title.as_str())?;
        Ok(())
    }
}

pub fn show_error_message(error: &Error) {
    show_simple_message_box(MessageBoxFlag::ERROR, "Error", error.to_string().as_str(), None).unwrap();
}
