use std::time::Duration;

use sdl3::{
    Sdl, VideoSubsystem,
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::WindowContext,
};

pub struct Window {
    sdl_context: Sdl,
    sdl_video: VideoSubsystem,
    window: sdl3::video::Window,
    window_canvas: Canvas<sdl3::video::Window>,
    texture_creator: TextureCreator<WindowContext>,

    width: u32,
    height: u32,
}

pub trait EventHandler {
    fn load(&mut self, window: &Window);
    fn update(&mut self, window: &Window);
    fn draw(&mut self, window: &Window);
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
        })
    }

    pub fn run(&mut self, handler: &mut impl EventHandler) -> anyhow::Result<()> {
        handler.load(self);

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
                            //self.next()?;
                        }
                        if y > 0.0 {
                            //self.prev()?;
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        //self.next()?;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        //self.prev()?;
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
}
